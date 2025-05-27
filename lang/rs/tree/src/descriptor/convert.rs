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
                .hoist(|context| Ok((literal.convert(context)?, literal.to_span())))?
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_descriptor_alias() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
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
            RSReference {
                id: GTReferenceId("module".into(), (0, 1).into()),
                identifier: "Name".into(),
                definition_id: GTDefinitionId("module".into(), "Name".into())
            }
            .into()
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })]
        );
    }

    #[test]
    fn test_convert_descriptor_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDescriptor::Vec(Box::new(RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_descriptor_inline_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
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
            RSDescriptor::InlineUse(RSInlineUse {
                path: RSPath("path/to/module".into(), "super::path::to::module".into()),
                name: "Name".into()
            })
        );
    }

    #[test]
    fn test_convert_descriptor_object() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
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
            RSDescriptor::Reference(
                RSReference {
                    id: GTReferenceId("module".into(), (0, 1).into()),
                    identifier: "Person".into(),
                    definition_id: GTDefinitionId("module".into(), "Person".into())
                }
                .into()
            )
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Person".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSPrimitive::String.into(),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![
                            r#"serde(default, skip_serializing_if = "Option::is_none")"#.into()
                        ],
                        name: "age".into(),
                        descriptor: RSOption::new(RSPrimitive::Int32.into()).into(),
                    }
                ]
                .into(),
            })]
        );
    }

    #[test]
    fn test_convert_descriptor_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSDescriptor::Primitive(RSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_descriptor_reference() {
        assert_eq!(
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
            RSReference {
                id: GTReferenceId("module".into(), (0, 1).into()),
                identifier: "Name".into(),
                definition_id: GTDefinitionId("module".into(), "Name".into())
            }
            .into()
        );
    }

    #[test]
    fn test_convert_descriptor_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDescriptor::Tuple(RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_descriptor_union() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: (0, 1).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut context)
            .unwrap(),
            RSDescriptor::Reference(
                RSReference {
                    id: GTReferenceId("module".into(), (0, 1).into()),
                    identifier: "Union".into(),
                    definition_id: GTDefinitionId("module".into(), "Union".into())
                }
                .into()
            )
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Enum(RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    "serde(untagged)".into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        name: "Boolean".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::Boolean).into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        name: "String".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::String).into()
                        ),
                    }
                ],
            })]
        );
    }

    // Record key

    #[test]
    fn test_convert_record_key() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String),
            GTRecordKey::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int8),
            GTRecordKey::Int8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int16),
            GTRecordKey::Int16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int32),
            GTRecordKey::Int32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int64),
            GTRecordKey::Int64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int128),
            GTRecordKey::Int128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntSize),
            GTRecordKey::IntSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU8),
            GTRecordKey::IntU8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU16),
            GTRecordKey::IntU16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU32),
            GTRecordKey::IntU32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU64),
            GTRecordKey::IntU64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU128),
            GTRecordKey::IntU128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntUSize),
            GTRecordKey::IntUSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Float32),
            GTRecordKey::Float32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Float64),
            GTRecordKey::Float64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean),
            GTRecordKey::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
    }
}
