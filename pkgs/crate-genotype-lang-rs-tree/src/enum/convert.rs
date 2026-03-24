use crate::prelude::internal::*;
use std::collections::HashSet;

impl RSConvert<RSEnum> for GTUnion {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSEnum> {
        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(None)
        };
        let id = context.build_definition_id(&name);
        context.drop_definition_id();
        context.enter_parent(RSContextParent::Definition(name.clone()));

        let mut variant_names: HashSet<RSIdentifier> = HashSet::new();

        let mut variants = self
            .descriptors
            .iter()
            .map(|descriptor| convert_variant(descriptor, &mut variant_names, context))
            .collect::<Result<Vec<_>>>()?;

        trim_variant_names(&name, &mut variants, &mut variant_names);

        if context.config().derive.contains(&"Default".into()) {
            let default_attrs = variants
                .iter()
                .flat_map(|variant| variant.attributes.iter().find(|attr| attr.0 == "default"));
            let count = default_attrs.clone().count();
            if count == 0 {
                return Err(RSConverterError::MissingDefaultVariant(self.span.clone()).into());
            } else if count > 1 {
                return Err(RSConverterError::MultipleDefaultVariants(self.span.clone()).into());
            }
        }

        let r#enum = RSEnum {
            id,
            doc,
            name,
            attributes: vec![
                context
                    .render_derive(RSContextRenderDeriveMode::UnionEnum)
                    .into(),
                r#"serde(untagged)"#.into(),
            ],
            variants,
        };

        context.add_import(RSDependencyIdent::Serde, "Deserialize".into());
        context.add_import(RSDependencyIdent::Serde, "Serialize".into());

        context.exit_parent();
        Ok(r#enum)
    }
}

fn convert_variant(
    descriptor: &GTDescriptor,
    variant_names: &mut HashSet<RSIdentifier>,
    context: &mut RSConvertContext,
) -> Result<RSEnumVariant> {
    let mut attributes = vec![];
    let variant_name = name_variant_descriptor(descriptor, context)?;
    let variant_name = ensure_unique_variant_name(variant_name, variant_names);

    context.enter_parent(RSContextParent::EnumVariant(variant_name.clone()));

    if GTAttribute::find_flag(descriptor.attributes(), "default") {
        attributes.push(RSAttribute("default".into()));
    }

    let descriptor = match descriptor {
        GTDescriptor::Literal(literal) => {
            let str = render_literal(literal);
            attributes.push(RSAttribute(format!("literal({str})",)));
            context.add_import(RSDependencyIdent::Litty, "literal".into());
            None
        }

        _ => Some(RSEnumVariantDescriptor::Descriptor(
            descriptor.convert(context)?,
        )),
    };

    let enum_variant = RSEnumVariant {
        doc: None,
        attributes,
        name: variant_name,
        descriptor,
    };

    context.exit_parent();
    Ok(enum_variant)
}

fn trim_variant_names(
    enum_name: &RSIdentifier,
    variants: &mut Vec<RSEnumVariant>,
    variant_names: &mut HashSet<RSIdentifier>,
) {
    for variant in variants.iter_mut() {
        if variant.name.0.starts_with(enum_name.0.as_ref()) {
            if let Some(trimmed_name) = variant.name.0.strip_prefix(enum_name.0.as_ref()) {
                let trimmed_name = RSIdentifier(trimmed_name.into());
                if !variant_names.contains(&trimmed_name) {
                    variant_names.remove(&variant.name);
                    variant_names.insert(trimmed_name.clone());
                    variant.name = trimmed_name;
                }
            }
        }
    }
}

fn ensure_unique_variant_name(
    variant_name: RSIdentifier,
    variant_names: &mut HashSet<RSIdentifier>,
) -> RSIdentifier {
    let name = if !variant_names.contains(&variant_name) {
        variant_name
    } else {
        enumerated_name(&variant_name, variant_names)
    };

    variant_names.insert(name.clone());

    name
}

fn enumerated_name(name: &RSIdentifier, variant_names: &HashSet<RSIdentifier>) -> RSIdentifier {
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
    descriptor: &GTDescriptor,
    context: &mut RSConvertContext,
) -> Result<RSIdentifier> {
    // If `#[variant = "<name>"]` is present, use it as the variant name
    if let Some(name) = GTAttribute::find_property_in(&descriptor.attributes(), "variant") {
        return Ok(name.into());
    }

    Ok(match descriptor {
        GTDescriptor::Alias(alias) => alias.name.convert(context)?,
        GTDescriptor::Reference(reference) => reference.identifier.convert(context)?,
        GTDescriptor::InlineImport(import) => import.name.convert(context)?,
        GTDescriptor::Object(object) => object.name.to_identifier().convert(context)?,
        GTDescriptor::Literal(literal) => RSConvertNameSegment::Literal(literal.clone())
            .render(true)
            .into(),
        GTDescriptor::Branded(branded) => branded.name.convert(context)?,
        GTDescriptor::Primitive(primitive) => match primitive.kind {
            GTPrimitiveKind::Boolean => "Boolean".into(),
            GTPrimitiveKind::String => "String".into(),
            GTPrimitiveKind::Number => "Number".into(),
            GTPrimitiveKind::Int8 => "Int8".into(),
            GTPrimitiveKind::Int16 => "Int16".into(),
            GTPrimitiveKind::Int32 => "Int32".into(),
            GTPrimitiveKind::Int64 => "Int".into(),
            GTPrimitiveKind::Int128 => "Int128".into(),
            GTPrimitiveKind::IntSize => "IntSize".into(),
            GTPrimitiveKind::IntU8 => "IntU8".into(),
            GTPrimitiveKind::IntU16 => "IntU16".into(),
            GTPrimitiveKind::IntU32 => "IntU32".into(),
            GTPrimitiveKind::IntU64 => "IntU64".into(),
            GTPrimitiveKind::IntU128 => "IntU128".into(),
            GTPrimitiveKind::IntUSize => "IntUSize".into(),
            GTPrimitiveKind::Float32 => "Float32".into(),
            GTPrimitiveKind::Float64 => "Float".into(),
        },
        GTDescriptor::Array(_) => "Vec".into(),
        GTDescriptor::Union(_) => "Union".into(),
        GTDescriptor::Record(_) => "Map".into(),
        GTDescriptor::Tuple(_) => "Tuple".into(),
        GTDescriptor::Any(_) => "Any".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Union"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Boolean"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
          ],
        )
        "#
        );

        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Serde, RSIdentifier("Deserialize")),
          (Serde, RSIdentifier("Serialize")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        context.provide_doc(Some("Hello, world!".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Union"),
          doc: Some(RSDoc("Hello, world!", false)),
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Union"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Boolean"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_variants() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("AnimalKind".into()));

        let union = parse_get_named::<GTUnion>(
            "AnimalKind",
            r#"
            AnimalKind: "dog" | "cat" | "bird"
            "#,
        );
        assert_ron_snapshot!(
            union.convert(&mut context).unwrap(),
            @r#"
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "AnimalKind"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("AnimalKind"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"dog\")"),
              ],
              name: RSIdentifier("Dog"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"cat\")"),
              ],
              name: RSIdentifier("Cat"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"bird\")"),
              ],
              name: RSIdentifier("Bird"),
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
          RSUse(
            dependency: Litty,
            reference: Named([
              Name(RSIdentifier("literal")),
            ]),
          ),
          RSUse(
            dependency: Serde,
            reference: Named([
              Name(RSIdentifier("Deserialize")),
              Name(RSIdentifier("Serialize")),
            ]),
          ),
        ]
        "#);
    }

    #[test]
    fn test_unique_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Union"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple"),
              descriptor: Some(Descriptor(Tuple(RSTuple(
                descriptors: [],
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple2"),
              descriptor: Some(Descriptor(Tuple(RSTuple(
                descriptors: [],
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String2"),
              descriptor: Some(Descriptor(Primitive(String))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple3"),
              descriptor: Some(Descriptor(Tuple(RSTuple(
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
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Union"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Int32"),
              descriptor: Some(Descriptor(Primitive(Int32))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Int"),
              descriptor: Some(Descriptor(Primitive(Int64))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("IntSize"),
              descriptor: Some(Descriptor(Primitive(IntSize))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Float32"),
              descriptor: Some(Descriptor(Primitive(Float32))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Float"),
              descriptor: Some(Descriptor(Primitive(Float64))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_names() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Union"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Union"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(null)"),
              ],
              name: RSIdentifier("Null"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"Hello\")"),
              ],
              name: RSIdentifier("Hello"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(true)"),
              ],
              name: RSIdentifier("True"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_integer_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Version"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(0)"),
              ],
              name: RSIdentifier("Lit0"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(1)"),
              ],
              name: RSIdentifier("Lit1"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_float_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Version"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(1.2)"),
              ],
              name: RSIdentifier("Lit1_2"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(3.4)"),
              ],
              name: RSIdentifier("Lit3_4"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_invalid_string_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));

        let union = parse_get_named::<GTUnion>(
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
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Version"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"0\")"),
              ],
              name: RSIdentifier("Lit0"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"1\")"),
              ],
              name: RSIdentifier("Lit1"),
              descriptor: None,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_trimmed_variant_names() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("ServerMessage".into()));

        let union = parse_get_named::<GTUnion>(
            "ServerMessage",
            r#"
            ServerMessage: ServerMessagePing | ServerMessagePong
            ServerMessagePing: { kind: "ping" }
            ServerMessagePong: { kind: "pong" }
            "#,
        );
        assert_ron_snapshot!(
            union.convert(&mut context).unwrap(),
            @r#"
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "ServerMessage"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("ServerMessage"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Ping"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(28, 45)),
                identifier: RSIdentifier("ServerMessagePing"),
                definition_id: GTDefinitionId(GTModuleId("module"), "ServerMessagePing"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Pong"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(48, 65)),
                identifier: RSIdentifier("ServerMessagePong"),
                definition_id: GTDefinitionId(GTModuleId("module"), "ServerMessagePong"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_trimmed_variant_names_conflicts() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("ServerMessage".into()));

        let union = parse_get_named::<GTUnion>(
            "ServerMessage",
            r#"
            ServerMessage: ServerMessagePing | ServerMessagePong | Ping
            ServerMessagePing: { kind: "ping" }
            ServerMessagePong: { kind: "pong" }
            Ping: string
            "#,
        );
        assert_ron_snapshot!(
            union.convert(&mut context).unwrap(),
            @r#"
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "ServerMessage"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("ServerMessage"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("ServerMessagePing"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(28, 45)),
                identifier: RSIdentifier("ServerMessagePing"),
                definition_id: GTDefinitionId(GTModuleId("module"), "ServerMessagePing"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Pong"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(48, 65)),
                identifier: RSIdentifier("ServerMessagePong"),
                definition_id: GTDefinitionId(GTModuleId("module"), "ServerMessagePong"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Ping"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(68, 72)),
                identifier: RSIdentifier("Ping"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Ping"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_attr_variant_assignment() {
        let mut context = Gtrs::convert_context_with_parent("Status");
        let union = Gt::union(descriptor_nodes![
            node_with!(
                Gt::alias("Hello", Gt::primitive_string()),
                attributes = vec![attribute_node!(variant = "Alias")]
            ),
            node_with!(
                Gt::array(Gt::primitive_boolean()),
                attributes = vec![attribute_node!(variant = "Array")]
            ),
            node_with!(
                Gt::inline_import("src/module", "Type"),
                attributes = vec![attribute_node!(variant = "Import")]
            ),
            node_with!(
                Gt::literal_string("ok"),
                attributes = vec![attribute_node!(variant = "Literal")]
            ),
            node_with!(
                Gt::object(
                    "Status",
                    descriptor_nodes![Gt::property("kind", Gt::primitive_string())]
                ),
                attributes = vec![attribute_node!(variant = "Object")]
            ),
            node_with!(
                Gt::primitive_boolean(),
                attributes = vec![attribute_node!(variant = "Primitive")]
            ),
            node_with!(
                Gt::reference("Hello"),
                attributes = vec![attribute_node!(variant = "Reference")]
            ),
            node_with!(
                Gt::tuple(descriptor_nodes![
                    Gt::primitive_string(),
                    Gt::primitive_f64(),
                ]),
                attributes = vec![attribute_node!(variant = "Tuple")]
            ),
            node_with!(
                Gt::union(descriptor_nodes![
                    Gt::primitive_string(),
                    Gt::primitive_f64(),
                ]),
                attributes = vec![attribute_node!(variant = "Union")]
            ),
            node_with!(
                Gt::record(Gt::record_key_string(), Gt::primitive_f64()),
                attributes = vec![attribute_node!(variant = "Record")]
            ),
            node_with!(
                Gt::any(),
                attributes = vec![attribute_node!(variant = "Whatever")]
            ),
            node_with!(
                Gt::branded("StatusStr", Gt::primitive_string()),
                attributes = vec![attribute_node!(variant = "Branded")]
            )
        ]);
        assert_ron_snapshot!(
            convert_node_with(union, &mut context),
            @r#"
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Status"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Status"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Alias"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("Hello"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Hello"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Array"),
              descriptor: Some(Descriptor(Vec(RSVec(
                descriptor: Primitive(Boolean),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Import"),
              descriptor: Some(Descriptor(InlineUse(RSInlineUse(
                path: RSPath(GTModuleId("path/to/module"), "src::module"),
                name: RSIdentifier("Type"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [
                RSAttribute("literal(\"ok\")"),
              ],
              name: RSIdentifier("Literal"),
              descriptor: None,
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Object"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("Status"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Status"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Primitive"),
              descriptor: Some(Descriptor(Primitive(Boolean))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Reference"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("Hello"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Hello"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple"),
              descriptor: Some(Descriptor(Tuple(RSTuple(
                descriptors: [
                  Primitive(String),
                  Primitive(Float64),
                ],
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Union"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("Status"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Status"),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Record"),
              descriptor: Some(Descriptor(Map(RSMap(
                key: Primitive(String),
                descriptor: Primitive(Float64),
              )))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Whatever"),
              descriptor: Some(Descriptor(Any(RSAny))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Branded"),
              descriptor: Some(Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("StatusStr"),
                definition_id: GTDefinitionId(GTModuleId("module"), "StatusStr"),
              )))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_attr_default() {
        let mut context = Gtrs::convert_context_with_parent("Status");
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(descriptor_nodes![
            Gt::primitive_string(),
            node_with!(
                Gt::primitive_number(),
                attributes = vec![attribute_node!(default)]
            ),
        ]);
        assert_debug_snapshot!(
            convert_node_with(union, &mut context),
            @r#"
        RSEnum {
            id: GTDefinitionId(
                GTModuleId(
                    "module",
                ),
                "Status",
            ),
            doc: None,
            attributes: [
                RSAttribute(
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)",
                ),
                RSAttribute(
                    "serde(untagged)",
                ),
            ],
            name: RSIdentifier(
                "Status",
            ),
            variants: [
                RSEnumVariant {
                    doc: None,
                    attributes: [],
                    name: RSIdentifier(
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
                RSEnumVariant {
                    doc: None,
                    attributes: [
                        RSAttribute(
                            "default",
                        ),
                    ],
                    name: RSIdentifier(
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
        let mut context = Gtrs::convert_context_with_parent("Status");
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(descriptor_nodes![
            Gt::primitive_string(),
            Gt::primitive_number()
        ]);
        assert_debug_snapshot!(
            convert_node_err_with(union, &mut context),
            @"
        MissingDefaultVariant(
            GTSpan(
                0,
                0,
            ),
        )
        "
        );
    }

    #[test]
    fn test_attr_default_multiple_err() {
        let mut context = Gtrs::convert_context_with_parent("Status");
        let mut config = RsConfigLang::default();
        config.derive.push("Default".into());
        context.assign_config(config);
        let union = Gt::union(descriptor_nodes![
            node_with!(
                Gt::primitive_string(),
                attributes = vec![attribute_node!(default)]
            ),
            node_with!(
                Gt::primitive_number(),
                attributes = vec![attribute_node!(default)]
            )
        ]);
        assert_debug_snapshot!(
            convert_node_err_with(union, &mut context),
            @"
        MultipleDefaultVariants(
            GTSpan(
                0,
                0,
            ),
        )
        "
        );
    }
}
