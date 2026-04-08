use crate::prelude::internal::*;
use std::collections::HashSet;

impl RsConvert<RsEnum> for GtUnion {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsEnum> {
        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(None)
        };
        let id = context.build_definition_id(&name);
        context.drop_definition_id();
        context.enter_parent(RsContextParent::Definition(name.clone()));

        let mut variant_names: HashSet<RsIdentifier> = HashSet::new();

        let mut literals_count = 0;
        let mut variants = self
            .descriptors
            .iter()
            .map(|descriptor| {
                convert_variant(descriptor, &mut variant_names, &mut literals_count, context)
            })
            .collect::<Result<Vec<_>>>()?;

        trim_variant_names(&name, &mut variants, &mut variant_names);

        if context.config().derive.contains(&"Default".into()) {
            let default_attrs = variants
                .iter()
                .flat_map(|variant| variant.attributes.iter().find(|attr| attr.0 == "default"));
            let count = default_attrs.clone().count();
            if count == 0 {
                return Err(RsConverterError::MissingDefaultVariant(self.span).into());
            } else if count > 1 {
                return Err(RsConverterError::MultipleDefaultVariants(self.span).into());
            }
        }

        let r#enum = RsEnum {
            id,
            doc,
            name,
            attributes: vec![
                {
                    // Use Litty derives instead of Serde if there are literal variants. It is
                    // a drop-in replacement and behaves the same for regular variants, but also
                    // adds support for literal variants.
                    let serde_mode = if literals_count > 0 {
                        RsContextRenderDeriveSerdeMode::Litty
                    } else {
                        RsContextRenderDeriveSerdeMode::Serde
                    };
                    context
                        .render_derive(RsContextRenderDeriveTypeMode::UnionEnum, serde_mode)
                        .into()
                },
                r#"serde(untagged)"#.into(),
            ],
            variants,
        };

        context.push_import(RsUse::new(RsDependencyIdent::Serde, "Deserialize".into()));
        context.push_import(RsUse::new(RsDependencyIdent::Serde, "Serialize".into()));

        context.exit_parent();
        Ok(r#enum)
    }
}

fn convert_variant(
    descriptor: &GtDescriptor,
    variant_names: &mut HashSet<RsIdentifier>,
    literals_count: &mut usize,
    context: &mut RsConvertContext,
) -> Result<RsEnumVariant> {
    let mut attributes = vec![];
    let variant_name = name_variant_descriptor(descriptor, context)?;
    let variant_name = ensure_unique_variant_name(variant_name, variant_names);

    context.enter_parent(RsContextParent::EnumVariant(variant_name.clone()));

    if GtAttribute::find_flag(descriptor.attributes(), "default") {
        attributes.push(RsAttribute("default".into()));
    }

    let descriptor = match descriptor {
        GtDescriptor::Literal(literal) => {
            let str = render_literal(literal);
            attributes.push(RsAttribute(format!("literal({str})",)));
            context.push_import(RsUse::new(RsDependencyIdent::Litty, "literal".into()));
            *literals_count += 1;
            None
        }

        _ => Some(RsEnumVariantDescriptor::Descriptor(
            descriptor.convert(context)?,
        )),
    };

    let enum_variant = RsEnumVariant {
        doc: None,
        attributes,
        name: variant_name,
        descriptor,
    };

    context.exit_parent();
    Ok(enum_variant)
}

fn trim_variant_names(
    enum_name: &RsIdentifier,
    variants: &mut [RsEnumVariant],
    variant_names: &mut HashSet<RsIdentifier>,
) {
    for variant in variants.iter_mut() {
        if variant.name.0.starts_with(enum_name.0.as_ref())
            && let Some(trimmed_name) = variant.name.0.strip_prefix(enum_name.0.as_ref())
        {
            let trimmed_name = RsIdentifier(trimmed_name.into());
            if !variant_names.contains(&trimmed_name) {
                variant_names.remove(&variant.name);
                variant_names.insert(trimmed_name.clone());
                variant.name = trimmed_name;
            }
        }
    }
}

fn ensure_unique_variant_name(
    variant_name: RsIdentifier,
    variant_names: &mut HashSet<RsIdentifier>,
) -> RsIdentifier {
    let name = if !variant_names.contains(&variant_name) {
        variant_name
    } else {
        enumerated_name(&variant_name, variant_names)
    };

    variant_names.insert(name.clone());

    name
}

fn enumerated_name(name: &RsIdentifier, variant_names: &HashSet<RsIdentifier>) -> RsIdentifier {
    let mut index = 2;
    loop {
        let enumerated_name = format!("{}{}", name.0, index).into();
        if !variant_names.contains(&enumerated_name) {
            return enumerated_name;
        }
        index += 1;
    }
}

fn name_variant_descriptor(
    descriptor: &GtDescriptor,
    context: &mut RsConvertContext,
) -> Result<RsIdentifier> {
    // If `#[variant = "<name>"]` is present, use it as the variant name
    if let Some(name) = GtAttribute::find_property_in(descriptor.attributes(), "variant") {
        return Ok(name.into());
    }

    Ok(match descriptor {
        GtDescriptor::Alias(alias) => alias.name.convert(context)?,
        GtDescriptor::Reference(reference) => reference.identifier.convert(context)?,
        GtDescriptor::InlineImport(import) => import.name.convert(context)?,
        GtDescriptor::Object(object) => object.name.to_identifier().convert(context)?,
        GtDescriptor::Literal(literal) => RsConvertNameSegment::Literal(literal.clone())
            .render(true)
            .into(),
        GtDescriptor::Branded(branded) => branded.name.convert(context)?,
        GtDescriptor::Primitive(primitive) => match primitive.kind {
            GtPrimitiveKind::Boolean => "Boolean".into(),
            GtPrimitiveKind::String => "String".into(),
            GtPrimitiveKind::Number => "Number".into(),
            GtPrimitiveKind::Int8 => "Int8".into(),
            GtPrimitiveKind::Int16 => "Int16".into(),
            GtPrimitiveKind::Int32 => "Int32".into(),
            GtPrimitiveKind::Int64 => "Int".into(),
            GtPrimitiveKind::Int128 => "Int128".into(),
            GtPrimitiveKind::IntSize => "IntSize".into(),
            GtPrimitiveKind::IntU8 => "IntU8".into(),
            GtPrimitiveKind::IntU16 => "IntU16".into(),
            GtPrimitiveKind::IntU32 => "IntU32".into(),
            GtPrimitiveKind::IntU64 => "IntU64".into(),
            GtPrimitiveKind::IntU128 => "IntU128".into(),
            GtPrimitiveKind::IntUSize => "IntUSize".into(),
            GtPrimitiveKind::Float32 => "Float32".into(),
            GtPrimitiveKind::Float64 => "Float".into(),
        },
        GtDescriptor::Array(_) => "Vec".into(),
        GtDescriptor::Union(_) => "Union".into(),
        GtDescriptor::Record(_) => "Map".into(),
        GtDescriptor::Tuple(_) => "Tuple".into(),
        GtDescriptor::Any(_) => "Any".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Union".into()));

        let union = parse_get_named::<GtUnion>(
            "Union",
            r#"
            Union: boolean | string
            "#,
        );

        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Union"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Boolean"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
          ],
        )
        "#
        );

        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Serde,
            reference: Named([
              Name(RsIdentifier("Deserialize")),
            ]),
          ),
          RsUse(
            dependency: Serde,
            reference: Named([
              Name(RsIdentifier("Serialize")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Union".into()));
        context.provide_doc(Some("Hello, world!".into()));

        let union = parse_get_named::<GtUnion>(
            "Union",
            r#"
            Union: boolean | string
            "#,
        );

        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Union"),
          doc: Some(RsDoc("Hello, world!", false)),
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Union"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Boolean"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_variants() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("AnimalKind".into()));

        let union = parse_get_named::<GtUnion>(
            "AnimalKind",
            r#"
            AnimalKind: "dog" | "cat" | "bird"
            "#,
        );
        assert_ron_snapshot!(
            union.convert(&mut context).unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "AnimalKind"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("AnimalKind"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"dog\")"),
              ],
              name: RsIdentifier("Dog"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"cat\")"),
              ],
              name: RsIdentifier("Cat"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"bird\")"),
              ],
              name: RsIdentifier("Bird"),
              descriptor: None,
            ),
          ],
        )
        "#);

        let imports = context.drain_imports();
        assert_ron_snapshot!(
            imports,
            @r#"
        [
          RsUse(
            dependency: Litty,
            reference: Named([
              Name(RsIdentifier("literal")),
            ]),
          ),
          RsUse(
            dependency: Serde,
            reference: Named([
              Name(RsIdentifier("Deserialize")),
              Name(RsIdentifier("Serialize")),
            ]),
          ),
        ]
        "#);
    }

    #[test]
    fn test_unique_name() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Union".into()));

        let union = parse_get_named::<GtUnion>(
            "Union",
            r#"
            Union: () | string | () | string | ()
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Union"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Tuple"),
              descriptor: Some(Descriptor(Tuple(RsTuple(
                descriptors: [],
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Tuple2"),
              descriptor: Some(Descriptor(Tuple(RsTuple(
                descriptors: [],
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("String2"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Tuple3"),
              descriptor: Some(Descriptor(Tuple(RsTuple(
                descriptors: [],
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_numeric_name() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Union".into()));

        let union = parse_get_named::<GtUnion>(
            "Union",
            r#"
            Union: i32 | i64 | isize | f32 | f64
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Union"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Int32"),
              descriptor: Some(Descriptor(Primitive(Int32))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Int"),
              descriptor: Some(Descriptor(Primitive(Int64))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("IntSize"),
              descriptor: Some(Descriptor(Primitive(IntSize))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Float32"),
              descriptor: Some(Descriptor(Primitive(Float32))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Float"),
              descriptor: Some(Descriptor(Primitive(Float64))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_names() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Union".into()));

        let union = parse_get_named::<GtUnion>(
            "Union",
            r#"
            Union: null | "Hello" | true
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Union"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(null)"),
              ],
              name: RsIdentifier("Null"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"Hello\")"),
              ],
              name: RsIdentifier("Hello"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(true)"),
              ],
              name: RsIdentifier("True"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_integer_name() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Version".into()));

        let union = parse_get_named::<GtUnion>(
            "Version",
            r#"
            Version: 0 | 1
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Version"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(0)"),
              ],
              name: RsIdentifier("Lit0"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(1)"),
              ],
              name: RsIdentifier("Lit1"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_float_name() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Version".into()));

        let union = parse_get_named::<GtUnion>(
            "Version",
            r#"
            Version: 1.2 | 3.4
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Version"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(1.2)"),
              ],
              name: RsIdentifier("Lit1_2"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(3.4)"),
              ],
              name: RsIdentifier("Lit3_4"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_invalid_string_name() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Version".into()));

        let union = parse_get_named::<GtUnion>(
            "Version",
            r#"
            Version: "0" | "1"
            "#,
        );
        assert_ron_snapshot!(
            union
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Version"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"0\")"),
              ],
              name: RsIdentifier("Lit0"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"1\")"),
              ],
              name: RsIdentifier("Lit1"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_trimmed_variant_names() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![
                (
                    Gt::reference_id((0, 1)),
                    Gt::definition_id("ServerMessagePing"),
                ),
                (
                    Gt::reference_id((0, 2)),
                    Gt::definition_id("ServerMessagePong"),
                ),
            ],
        );
        context.enter_parent(RsContextParent::Alias("ServerMessage".into()));

        let union = Gt::union(vec_into![
            Gt::reference("ServerMessagePing", (0, 1)),
            Gt::reference("ServerMessagePong", (0, 2))
        ]);

        assert_ron_snapshot!(
            convert_node_with(union, &mut context),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "ServerMessage"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("ServerMessage"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Ping"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 1)),
                identifier: RsIdentifier("ServerMessagePing"),
                definition_id: GtDefinitionId(GtModuleId("module"), "ServerMessagePing"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Pong"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 2)),
                identifier: RsIdentifier("ServerMessagePong"),
                definition_id: GtDefinitionId(GtModuleId("module"), "ServerMessagePong"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_trimmed_variant_names_conflicts() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![
                (
                    Gt::reference_id((0, 1)),
                    Gt::definition_id("ServerMessagePing"),
                ),
                (
                    Gt::reference_id((0, 2)),
                    Gt::definition_id("ServerMessagePong"),
                ),
                (Gt::reference_id((0, 3)), Gt::definition_id("Ping")),
            ],
        );
        context.enter_parent(Rst::context_parent("ServerMessage"));

        let union = Gt::union(vec_into![
            Gt::reference("ServerMessagePing", (0, 1)),
            Gt::reference("ServerMessagePong", (0, 2)),
            Gt::reference("Ping", (0, 3)),
        ]);

        assert_ron_snapshot!(
            union.convert(&mut context).unwrap(),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "ServerMessage"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("ServerMessage"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("ServerMessagePing"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 1)),
                identifier: RsIdentifier("ServerMessagePing"),
                definition_id: GtDefinitionId(GtModuleId("module"), "ServerMessagePing"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Pong"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 2)),
                identifier: RsIdentifier("ServerMessagePong"),
                definition_id: GtDefinitionId(GtModuleId("module"), "ServerMessagePong"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Ping"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 3)),
                identifier: RsIdentifier("Ping"),
                definition_id: GtDefinitionId(GtModuleId("module"), "Ping"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_attr_variant_assignment() {
        let mut context = Rst::convert_context_with(
            vec![(Gt::path_module_id((0, 1)), "module/path".into())],
            vec![(Gt::reference_id((0, 2)), Gt::definition_id("Hello"))],
        );
        context.enter_parent(Rst::context_parent("Status"));

        let union = Gt::union(vec_into![
            assign!(
                Gt::alias("Hello", Gt::primitive_string()),
                attributes = vec![attribute_node!(variant = "Alias")]
            ),
            assign!(
                Gt::array(Gt::primitive_boolean()),
                attributes = vec![attribute_node!(variant = "Array")]
            ),
            assign!(
                Gt::inline_import("src/module", "Type", (0, 1)),
                attributes = vec![attribute_node!(variant = "Import")]
            ),
            assign!(
                Gt::literal_string("ok"),
                attributes = vec![attribute_node!(variant = "Literal")]
            ),
            assign!(
                Gt::object(
                    "Status",
                    vec_into![Gt::property("kind", Gt::primitive_string())]
                ),
                attributes = vec![attribute_node!(variant = "Object")]
            ),
            assign!(
                Gt::primitive_boolean(),
                attributes = vec![attribute_node!(variant = "Primitive")]
            ),
            assign!(
                Gt::reference("Hello", (0, 2)),
                attributes = vec![attribute_node!(variant = "Reference")]
            ),
            assign!(
                Gt::tuple(vec_into![Gt::primitive_string(), Gt::primitive_f64(),]),
                attributes = vec![attribute_node!(variant = "Tuple")]
            ),
            assign!(
                Gt::union(vec_into![Gt::primitive_string(), Gt::primitive_f64(),]),
                attributes = vec![attribute_node!(variant = "Union")]
            ),
            assign!(
                Gt::record(Gt::record_key_string(), Gt::primitive_f64()),
                attributes = vec![attribute_node!(variant = "Record")]
            ),
            assign!(
                Gt::any(),
                attributes = vec![attribute_node!(variant = "Whatever")]
            ),
            assign!(
                Gt::branded("StatusStr", Gt::primitive_string()),
                attributes = vec![attribute_node!(variant = "Branded")]
            )
        ]);

        assert_ron_snapshot!(
            convert_node_with(union, &mut context),
            @r#"
        RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Status"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Status"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Alias"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                identifier: RsIdentifier("Hello"),
                definition_id: GtDefinitionId(GtModuleId("module"), "Hello"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Array"),
              descriptor: Some(Descriptor(Vec(RsVec(
                descriptor: Primitive(Boolean),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Import"),
              descriptor: Some(Descriptor(InlineUse(RsInlineUse(
                path: RsPath(GtModuleId("module/path"), "src::module"),
                name: RsIdentifier("Type"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [
                RsAttribute("literal(\"ok\")"),
              ],
              name: RsIdentifier("Literal"),
              descriptor: None,
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Object"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                identifier: RsIdentifier("Status"),
                definition_id: GtDefinitionId(GtModuleId("module"), "Status"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Primitive"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Reference"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 2)),
                identifier: RsIdentifier("Hello"),
                definition_id: GtDefinitionId(GtModuleId("module"), "Hello"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Tuple"),
              descriptor: Some(Descriptor(Tuple(RsTuple(
                descriptors: [
                  Primitive(String),
                  Primitive(Float64),
                ],
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Union"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                identifier: RsIdentifier("Status"),
                definition_id: GtDefinitionId(GtModuleId("module"), "Status"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Record"),
              descriptor: Some(Descriptor(Map(RsMap(
                key: Primitive(String),
                descriptor: Primitive(Float64),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Whatever"),
              descriptor: Some(Descriptor(Any(RsAny))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Branded"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                identifier: RsIdentifier("StatusStr"),
                definition_id: GtDefinitionId(GtModuleId("module"), "StatusStr"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_attr_default() {
        let mut context = Rst::convert_context();
        context.enter_parent(Rst::context_parent("Status"));
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(vec_into![
            Gt::primitive_string(),
            assign!(
                Gt::primitive_number(),
                attributes = vec![attribute_node!(default)]
            ),
        ]);
        assert_debug_snapshot!(
            convert_node_with(union, &mut context),
            @r#"
        RsEnum {
            id: GtDefinitionId(
                GtModuleId(
                    "module",
                ),
                "Status",
            ),
            doc: None,
            attributes: [
                RsAttribute(
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)",
                ),
                RsAttribute(
                    "serde(untagged)",
                ),
            ],
            name: RsIdentifier(
                "Status",
            ),
            variants: [
                RsEnumVariant {
                    doc: None,
                    attributes: [],
                    name: RsIdentifier(
                        "String",
                    ),
                    descriptor: Some(
                        Descriptor(
                            Primitive(
                                String,
                            ),
                        ),
                    ),
                },
                RsEnumVariant {
                    doc: None,
                    attributes: [
                        RsAttribute(
                            "default",
                        ),
                    ],
                    name: RsIdentifier(
                        "Number",
                    ),
                    descriptor: Some(
                        Descriptor(
                            Primitive(
                                Float64,
                            ),
                        ),
                    ),
                },
            ],
        }
        "#
        );
    }

    #[test]
    fn test_attr_default_missing_err() {
        let mut context = Rst::convert_context();
        context.enter_parent(Rst::context_parent("Status"));
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(vec_into![Gt::primitive_string(), Gt::primitive_number()]);
        assert_debug_snapshot!(
            convert_node_err_with(union, &mut context),
            @"
        MissingDefaultVariant(
            GtSpan(
                0,
                0,
            ),
        )
        "
        );
    }

    #[test]
    fn test_attr_default_multiple_err() {
        let mut context = Rst::convert_context();
        context.enter_parent(Rst::context_parent("Status"));
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(vec_into![
            assign!(
                Gt::primitive_string(),
                attributes = vec![attribute_node!(default)]
            ),
            assign!(
                Gt::primitive_number(),
                attributes = vec![attribute_node!(default)]
            )
        ]);
        assert_debug_snapshot!(
            convert_node_err_with(union, &mut context),
            @"
        MultipleDefaultVariants(
            GtSpan(
                0,
                0,
            ),
        )
        "
        );
    }
}
