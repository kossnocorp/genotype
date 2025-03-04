use genotype_lang_rs_tree::*;

use genotype_parser::tree::descriptor::GTDescriptor;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

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

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::naming::RSContextParent;

    use super::*;

    #[test]
    fn test_convert_alias() {
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
    fn test_convert_array() {
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
    fn test_convert_inline_import() {
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
    fn test_convert_object() {
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
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)".into()
                ],
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
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSOption::new(RSPrimitive::Int32.into()).into(),
                    }
                ]
                .into(),
            })]
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSDescriptor::Primitive(RSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
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
    fn test_convert_tuple() {
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
    fn test_convert_union() {
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
                    "derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)".into(),
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
}
