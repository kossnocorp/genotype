use crate::prelude::internal::*;

impl RSConvert<RSDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSDescriptor> {
        Ok(match self {
            GTDescriptor::Alias(alias) => context
                .hoist(|context| Ok((alias.convert(context)?, alias.span.clone())))?
                .into(),

            GTDescriptor::Array(array) => array.convert(context)?.into(),

            GTDescriptor::InlineImport(import) => import.convert(context)?.into(),

            GTDescriptor::Literal(literal) => context
                .hoist(|context| Ok((literal.convert(context)?, literal.span())))?
                .into(),

            GTDescriptor::Object(object) => context
                .hoist(|context| Ok((object.convert(context)?, object.span.clone())))?
                .into(),

            GTDescriptor::Primitive(primitive) => primitive.convert(context)?.into(),

            GTDescriptor::Record(record) => record.convert(context)?.into(),

            GTDescriptor::Reference(name) => name.convert(context)?.into(),

            GTDescriptor::Tuple(tuple) => tuple.convert(context)?.into(),

            GTDescriptor::Union(union) => context
                .hoist(|context| Ok((union.convert(context)?, union.span.clone())))?
                .into(),

            GTDescriptor::Any(any) => any.convert(context)?.into(),

            GTDescriptor::Branded(branded) => context
                .hoist(|context| Ok((branded.convert(context)?, branded.span.clone())))?
                .into(),
        })
    }
}

impl RSConvert<RSDescriptor> for GTRecordKey {
    fn convert(&self, _context: &mut RSConvertContext) -> Result<RSDescriptor> {
        Ok(match self {
            GTRecordKey::String(_) => RSPrimitive::String.into(),
            GTRecordKey::Number(_) => RSPrimitive::Float64.into(),
            GTRecordKey::Int8(_) => RSPrimitive::Int8.into(),
            GTRecordKey::Int16(_) => RSPrimitive::Int16.into(),
            GTRecordKey::Int32(_) => RSPrimitive::Int32.into(),
            GTRecordKey::Int64(_) => RSPrimitive::Int64.into(),
            GTRecordKey::Int128(_) => RSPrimitive::Int128.into(),
            GTRecordKey::IntSize(_) => RSPrimitive::IntSize.into(),
            GTRecordKey::IntU8(_) => RSPrimitive::IntU8.into(),
            GTRecordKey::IntU16(_) => RSPrimitive::IntU16.into(),
            GTRecordKey::IntU32(_) => RSPrimitive::IntU32.into(),
            GTRecordKey::IntU64(_) => RSPrimitive::IntU64.into(),
            GTRecordKey::IntU128(_) => RSPrimitive::IntU128.into(),
            GTRecordKey::IntUSize(_) => RSPrimitive::IntUSize.into(),
            GTRecordKey::Float32(_) => RSPrimitive::Float32.into(),
            GTRecordKey::Float64(_) => RSPrimitive::Float64.into(),
            GTRecordKey::Boolean(_) => RSPrimitive::Boolean.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_descriptor_alias() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 1).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Name"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Name"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(RSAlias(
            id: GTDefinitionId(GTModuleId("module"), "Name"),
            doc: None,
            name: RSIdentifier("Name"),
            descriptor: Primitive(Boolean),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_array() {
        assert_ron_snapshot!(
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @"
        Vec(RSVec(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_descriptor_inline_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("path/to/module".into()),
                    "./path/to/module".into()
                ),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut context)
            .unwrap(),
            @r#"
        InlineUse(RSInlineUse(
          path: RSPath(GTModuleId("path/to/module"), "super::path::to::module"),
          name: RSIdentifier("Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_object() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTDescriptor::Object(GTObject {
                span: (0, 1).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int32((0, 0).into()).into(),
                        required: false,
                    }
                ],
            })
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Person"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Person"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Struct(RSStruct(
            id: GTDefinitionId(GTModuleId("module"), "Person"),
            doc: None,
            attributes: [
              RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            ],
            name: RSIdentifier("Person"),
            fields: Resolved([
              RSField(
                doc: None,
                attributes: [],
                name: RSFieldName("name"),
                descriptor: Primitive(String),
              ),
              RSField(
                doc: None,
                attributes: [
                  RSAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
                ],
                name: RSFieldName("age"),
                descriptor: Option(RSOption(
                  descriptor: Primitive(Int32),
                )),
              ),
            ]),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_primitive() {
        assert_ron_snapshot!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_descriptor_reference() {
        assert_ron_snapshot!(
            GTDescriptor::Reference(GTReference {
                span: (0, 1).into(),
                id: GTReferenceId("module".into(), (0, 1).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Name"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_tuple() {
        assert_ron_snapshot!(
            GTDescriptor::Tuple(GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @"
        Tuple(RSTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        ))
        "
        );
    }

    #[test]
    fn test_convert_descriptor_union() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_ron_snapshot!(
            GTDescriptor::Union(GTUnion {
                span: (0, 1).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Union"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Union"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Enum(RSEnum(
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
          )),
        ]
        "#
        );
    }

    // Record key

    #[test]
    fn test_convert_record_key() {
        assert_ron_snapshot!(
            GTRecordKey::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(String)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Int8)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Int16)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Int32)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Int64)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Int128)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntSize)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntU8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntU8)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntU16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntU16)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntU32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntU32)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntU64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntU64)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntU128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntU128)"
        );
        assert_ron_snapshot!(
            GTRecordKey::IntUSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(IntUSize)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Float32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Float32)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Float64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Float64)"
        );
        assert_ron_snapshot!(
            GTRecordKey::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Primitive(Boolean)"
        );
    }
}
