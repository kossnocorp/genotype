use genotype_lang_ts_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSDescriptor> for GTDescriptor {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSDescriptor
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTDescriptor::Alias(alias) => {
                hoist(alias.convert(resolve, hoist));
                TSDescriptor::Reference(TSReference(alias.name.convert(resolve, hoist)))
            }

            GTDescriptor::Array(array) => {
                TSDescriptor::Array(Box::new(array.convert(resolve, hoist)))
            }

            GTDescriptor::InlineImport(import) => {
                TSDescriptor::InlineImport(import.convert(resolve, hoist))
            }

            GTDescriptor::Literal(literal) => {
                TSDescriptor::Literal(literal.convert(resolve, hoist))
            }

            GTDescriptor::Object(object) => {
                let descriptor = TSDescriptor::Object(object.convert(resolve, hoist));
                if object.extensions.is_empty() {
                    descriptor
                } else {
                    let mut descriptors: Vec<TSDescriptor> = vec![descriptor];
                    let extensions = object
                        .extensions
                        .iter()
                        .map(|extension| {
                            TSDescriptor::from(extension.reference.convert(resolve, hoist))
                        })
                        .collect::<Vec<TSDescriptor>>();
                    descriptors.extend(extensions);
                    TSDescriptor::Intersection(TSIntersection { descriptors })
                }
            }

            GTDescriptor::Primitive(primitive) => {
                TSDescriptor::Primitive(primitive.convert(resolve, hoist))
            }

            GTDescriptor::Reference(name) => TSDescriptor::Reference(name.convert(resolve, hoist)),

            GTDescriptor::Tuple(tuple) => TSDescriptor::Tuple(tuple.convert(resolve, hoist)),

            GTDescriptor::Union(union) => TSDescriptor::Union(union.convert(resolve, hoist)),

            GTDescriptor::Record(record) => {
                TSDescriptor::Record(Box::new(record.convert(resolve, hoist)))
            }

            GTDescriptor::Any(any) => TSDescriptor::Any(any.convert(resolve, hoist)),

            GTDescriptor::Branded(branded) => {
                let branded = branded.convert(resolve, hoist);
                let reference = branded.name.clone().into();
                hoist(branded.into());
                TSDescriptor::Reference(reference)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&TSConvertResolve::new(), &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            TSDescriptor::Reference("Name".into())
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
                .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
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
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Record(Box::new(TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }))
        );
    }

    #[test]
    fn test_convert_any() {
        assert_eq!(
            GTDescriptor::Any(GTAny((0, 0).into())).convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Any(TSAny)
        );
    }

    #[test]
    fn test_convert_branded() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTDescriptor::Branded(GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            })
            .convert(&TSConvertResolve::new(), &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            TSDescriptor::Reference("UserId".into())
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![TSDefinition::Branded(TSBranded {
                doc: None,
                name: "UserId".into(),
                primitive: TSPrimitive::String,
            })]
        );
    }
}
