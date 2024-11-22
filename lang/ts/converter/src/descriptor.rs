use genotype_lang_ts_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut TSConvertContext) -> TSDescriptor {
        match self {
            GTDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GTDescriptor::Array(array) => TSDescriptor::Array(Box::new(array.convert(context))),

            GTDescriptor::InlineImport(import) => {
                TSDescriptor::InlineImport(import.convert(context))
            }

            GTDescriptor::Literal(literal) => TSDescriptor::Literal(literal.convert(context)),

            GTDescriptor::Object(object) => {
                let descriptor = TSDescriptor::Object(object.convert(context));
                if object.extensions.is_empty() {
                    descriptor
                } else {
                    let mut descriptors: Vec<TSDescriptor> = vec![descriptor];
                    let extensions = object
                        .extensions
                        .iter()
                        .map(|extension| TSDescriptor::from(extension.reference.convert(context)))
                        .collect::<Vec<TSDescriptor>>();
                    descriptors.extend(extensions);
                    TSDescriptor::Intersection(TSIntersection { descriptors })
                }
            }

            GTDescriptor::Primitive(primitive) => {
                TSDescriptor::Primitive(primitive.convert(context))
            }

            GTDescriptor::Reference(name) => TSDescriptor::Reference(name.convert(context)),

            GTDescriptor::Tuple(tuple) => TSDescriptor::Tuple(tuple.convert(context)),

            GTDescriptor::Union(union) => TSDescriptor::Union(union.convert(context)),

            GTDescriptor::Record(record) => TSDescriptor::Record(Box::new(record.convert(context))),

            GTDescriptor::Any(any) => TSDescriptor::Any(any.convert(context)),

            GTDescriptor::Branded(branded) => {
                context.hoist(|context| branded.convert(context)).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let mut context = Default::default();
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut context),
            TSDescriptor::Reference("Name".into())
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),]
        );
    }

    #[test]
    fn test_convert_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut Default::default()),
            TSDescriptor::Array(Box::new(TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut Default::default()),
            TSDescriptor::InlineImport(TSInlineImport {
                path: "./path/to/module.ts".into(),
                name: "Name".into()
            })
        );
    }

    #[test]
    fn test_convert_object() {
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: (0, 0).into(),
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
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            })
            .convert(&mut Default::default()),
            TSDescriptor::Object(TSObject {
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSPrimitive::String.into(),
                        required: true,
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSUnion {
                            descriptors: vec![
                                TSPrimitive::Number.into(),
                                TSPrimitive::Undefined.into()
                            ]
                        }
                        .into(),
                        required: false,
                    }
                ]
            })
        );

        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: (0, 0).into(),
                name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                extensions: vec![GTExtension {
                    span: (0, 0).into(),
                    reference: GTReference {
                        span: (0, 0).into(),
                        id: GTReferenceId("module".into(), (0, 0).into()),
                        definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                            "module".into(),
                            "Good".into()
                        ),),
                        identifier: GTIdentifier::new((0, 0).into(), "Good".into())
                    }
                    .into()
                }],
                properties: vec![GTProperty {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    name: GTKey::new((0, 0).into(), "title".into()),
                    descriptor: GTPrimitive::String((0, 0).into()).into(),
                    required: true,
                },]
            })
            .convert(&mut Default::default()),
            TSDescriptor::Intersection(TSIntersection {
                descriptors: vec![
                    TSObject {
                        properties: vec![TSProperty {
                            doc: None,
                            name: "title".into(),
                            descriptor: TSPrimitive::String.into(),
                            required: true,
                        },]
                    }
                    .into(),
                    "Good".into()
                ]
            })
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut Default::default()),
            TSDescriptor::Primitive(TSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference(GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                ),),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut Default::default()),
            TSDescriptor::Reference("Name".into())
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
            .convert(&mut Default::default()),
            TSDescriptor::Tuple(TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_union() {
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut Default::default()),
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_record() {
        assert_eq!(
            GTDescriptor::Record(Box::new(GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }))
            .convert(&mut Default::default()),
            TSDescriptor::Record(Box::new(TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }))
        );
    }

    #[test]
    fn test_convert_any() {
        assert_eq!(
            GTDescriptor::Any(GTAny((0, 0).into())).convert(&mut Default::default()),
            TSDescriptor::Any(TSAny)
        );
    }

    #[test]
    fn test_convert_branded() {
        let mut context = Default::default();
        assert_eq!(
            GTDescriptor::Branded(GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            })
            .convert(&mut context),
            TSDescriptor::Reference("UserId".into())
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![TSDefinition::Branded(TSBranded {
                doc: None,
                name: "UserId".into(),
                primitive: TSPrimitive::String,
            })]
        );
    }
}
