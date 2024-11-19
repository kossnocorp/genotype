use genotype_lang_ts_tree::{alias::TSAlias, definition::TSDefinition, interface::TSInterface};
use genotype_parser::tree::{alias::GTAlias, descriptor::GTDescriptor};

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSDefinition> for GTAlias {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSDefinition
    where
        HoistFn: Fn(TSDefinition),
    {
        let doc = self.doc.as_ref().map(|d| d.convert(resolve, hoist));
        let name = self.name.convert(resolve, hoist);

        match &self.descriptor {
            GTDescriptor::Object(object) => TSDefinition::Interface(TSInterface {
                doc,
                name,
                extensions: object
                    .extensions
                    .iter()
                    .map(|e| e.convert(resolve, hoist))
                    .collect(),
                properties: object
                    .properties
                    .iter()
                    .map(|p| p.convert(resolve, hoist))
                    .collect(),
            }),

            _ => TSDefinition::Alias(TSAlias {
                doc,
                name,
                descriptor: self.descriptor.convert(resolve, hoist),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Book".into(),
                extensions: vec![],
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "title".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        doc: None,
                        name: "author".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }

    #[test]
    fn test_convert_extensions() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
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
                            )),
                            identifier: GTIdentifier::new((0, 0).into(), "Good".into())
                        }
                        .into()
                    }],
                    properties: vec![GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "author".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    }]
                })
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Book".into(),
                extensions: vec!["Good".into()],
                properties: vec![TSProperty {
                    doc: None,
                    name: "author".into(),
                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    required: true,
                }]
            }),
        );

        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GTObject {
                            span: (0, 0).into(),
                            name: GTObjectName::Alias(
                                GTIdentifier::new((0, 0).into(), "BookAuthorObj".into()),
                                GTObjectNameParent::Alias(GTIdentifier::new(
                                    (0, 0).into(),
                                    "BookAuthor".into()
                                ))
                            ),
                            extensions: vec![GTExtension {
                                span: (0, 0).into(),
                                reference: GTReference {
                                    span: (0, 0).into(),
                                    id: GTReferenceId("module".into(), (0, 0).into()),
                                    definition_id: GTReferenceDefinitionId::Resolved(
                                        GTDefinitionId("module".into(), "Good".into())
                                    ),
                                    identifier: GTIdentifier::new((0, 0).into(), "Good".into())
                                }
                                .into()
                            }],
                            properties: vec![GTProperty {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((0, 0).into(), "author".into()),
                                descriptor: GTPrimitive::String((0, 0).into()).into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GTPrimitive::String((0, 0).into()).into(),
                    ]
                })
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Book".into(),
                descriptor: TSUnion {
                    descriptors: vec![
                        TSIntersection {
                            descriptors: vec![
                                TSObject {
                                    properties: vec![TSProperty {
                                        doc: None,
                                        name: "author".into(),
                                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                        required: true,
                                    }]
                                }
                                .into(),
                                "Good".into()
                            ],
                        }
                        .into(),
                        TSPrimitive::String.into(),
                    ]
                }
                .into(),
            }),
        );
    }

    #[test]
    fn test_convert_doc_interface() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![]
                })
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Interface(TSInterface {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Book".into(),
                extensions: vec![],
                properties: vec![]
            }),
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Alias(TSAlias {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }
}
