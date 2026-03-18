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

        let variants = self
            .descriptors
            .iter()
            .map(|descriptor| convert_variant(descriptor, &mut variant_names, context))
            .collect::<Result<Vec<_>>>()?;

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
    let name = name_descriptor(descriptor, context)?;
    let name = ensure_unique_name(name, variant_names);

    context.enter_parent(RSContextParent::EnumVariant(name.clone()));

    let descriptor = RSEnumVariantDescriptor::Descriptor(descriptor.convert(context)?);

    let enum_variant = RSEnumVariant {
        doc: None,
        attributes: vec![],
        name,
        descriptor,
    };

    context.exit_parent();
    Ok(enum_variant)
}

fn ensure_unique_name(
    name: RSIdentifier,
    variant_names: &mut HashSet<RSIdentifier>,
) -> RSIdentifier {
    let name = if !variant_names.contains(&name) {
        name
    } else {
        enumerated_name(&name, variant_names)
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

fn name_descriptor(
    descriptor: &GTDescriptor,
    context: &mut RSConvertContext,
) -> Result<RSIdentifier> {
    Ok(match descriptor {
        GTDescriptor::Alias(alias) => alias.name.convert(context)?,
        GTDescriptor::Reference(reference) => reference.identifier.convert(context)?,
        GTDescriptor::InlineImport(import) => import.name.convert(context)?,
        GTDescriptor::Object(object) => object.name.to_identifier().convert(context)?,
        GTDescriptor::Literal(literal) => RSConvertNameSegment::Literal(literal.clone())
            .render(true)
            .into(),
        GTDescriptor::Branded(branded) => branded.name.convert(context)?,
        GTDescriptor::Primitive(primitive) => match primitive {
            GTPrimitive::Boolean(_) => "Boolean".into(),
            GTPrimitive::String(_) => "String".into(),
            GTPrimitive::Number(_) => "Number".into(),
            GTPrimitive::Int8(_) => "Int8".into(),
            GTPrimitive::Int16(_) => "Int16".into(),
            GTPrimitive::Int32(_) => "Int32".into(),
            GTPrimitive::Int64(_) => "Int".into(),
            GTPrimitive::Int128(_) => "Int128".into(),
            GTPrimitive::IntSize(_) => "IntSize".into(),
            GTPrimitive::IntU8(_) => "IntU8".into(),
            GTPrimitive::IntU16(_) => "IntU16".into(),
            GTPrimitive::IntU32(_) => "IntU32".into(),
            GTPrimitive::IntU64(_) => "IntU64".into(),
            GTPrimitive::IntU128(_) => "IntU128".into(),
            GTPrimitive::IntUSize(_) => "IntUSize".into(),
            GTPrimitive::Float32(_) => "Float32".into(),
            GTPrimitive::Float64(_) => "Float".into(),
            GTPrimitive::Null(_) => "Null".into(),
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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
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
              descriptor: Descriptor(Primitive(Boolean)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Descriptor(Primitive(String)),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
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
              name: RSIdentifier("String"),
              descriptor: Descriptor(Primitive(String)),
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
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
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
              name: RSIdentifier("String"),
              descriptor: Descriptor(Primitive(String)),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_naming() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Admin".into()));
        context.enter_parent(RSContextParent::Field("role".into()));
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::String((0, 1).into(), "superadmin".into()).into(),
                    GTLiteral::String((0, 2).into(), "admin".into()).into(),
                    GTLiteral::String((0, 3).into(), "moderator".into()).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "AdminRole"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("AdminRole"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Superadmin"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
                identifier: RSIdentifier("AdminRoleSuperadmin"),
                definition_id: GTDefinitionId(GTModuleId("module"), "AdminRoleSuperadmin"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Admin"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 2)),
                identifier: RSIdentifier("AdminRoleAdmin"),
                definition_id: GTDefinitionId(GTModuleId("module"), "AdminRoleAdmin"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Moderator"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 3)),
                identifier: RSIdentifier("AdminRoleModerator"),
                definition_id: GTDefinitionId(GTModuleId("module"), "AdminRoleModerator"),
              ))),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_unique_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into(),
                    GTPrimitive::String((0, 0).into()).into(),
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into(),
                    GTPrimitive::String((0, 0).into()).into(),
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into()
                ],
            }
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
              descriptor: Descriptor(Tuple(RSTuple(
                descriptors: [],
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Descriptor(Primitive(String)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple2"),
              descriptor: Descriptor(Tuple(RSTuple(
                descriptors: [],
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String2"),
              descriptor: Descriptor(Primitive(String)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Tuple3"),
              descriptor: Descriptor(Tuple(RSTuple(
                descriptors: [],
              ))),
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
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Int32((0, 0).into()).into(),
                    GTPrimitive::Int64((0, 0).into()).into(),
                    GTPrimitive::IntSize((0, 0).into()).into(),
                    GTPrimitive::Float32((0, 0).into()).into(),
                    GTPrimitive::Float64((0, 0).into()).into(),
                ],
            }
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
              descriptor: Descriptor(Primitive(Int32)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Int"),
              descriptor: Descriptor(Primitive(Int64)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("IntSize"),
              descriptor: Descriptor(Primitive(IntSize)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Float32"),
              descriptor: Descriptor(Primitive(Float32)),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Float"),
              descriptor: Descriptor(Primitive(Float64)),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_literal_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Null((0, 1).into()).into(),
                    GTLiteral::String((0, 2).into(), "Hello".into()).into(),
                    GTLiteral::Boolean((0, 3).into(), true).into(),
                ],
            }
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
              name: RSIdentifier("Null"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
                identifier: RSIdentifier("UnionNull"),
                definition_id: GTDefinitionId(GTModuleId("module"), "UnionNull"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Hello"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 2)),
                identifier: RSIdentifier("UnionHello"),
                definition_id: GTDefinitionId(GTModuleId("module"), "UnionHello"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("True"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 3)),
                identifier: RSIdentifier("UnionTrue"),
                definition_id: GTDefinitionId(GTModuleId("module"), "UnionTrue"),
              ))),
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
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Integer((0, 1).into(), 0).into(),
                    GTLiteral::Integer((0, 2).into(), 1).into(),
                ],
            }
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
              attributes: [],
              name: RSIdentifier("Lit0"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
                identifier: RSIdentifier("Version0"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version0"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Lit1"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 2)),
                identifier: RSIdentifier("Version1"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version1"),
              ))),
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
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Float((0, 1).into(), 1.2).into(),
                    GTLiteral::Float((0, 2).into(), 3.4).into(),
                ],
            }
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
              attributes: [],
              name: RSIdentifier("Lit1_2"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
                identifier: RSIdentifier("Version1_2"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version1_2"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Lit3_4"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 2)),
                identifier: RSIdentifier("Version3_4"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version3_4"),
              ))),
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
        assert_ron_snapshot!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::String((0, 1).into(), "0".into()).into(),
                    GTLiteral::String((0, 2).into(), "1".into()).into(),
                ],
            }
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
              attributes: [],
              name: RSIdentifier("Lit0"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
                identifier: RSIdentifier("Version0"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version0"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Lit1"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 2)),
                identifier: RSIdentifier("Version1"),
                definition_id: GTDefinitionId(GTModuleId("module"), "Version1"),
              ))),
            ),
          ],
        )
        "#
        );
    }
}
