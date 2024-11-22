use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::tree::module::GTModule;

use crate::{context::TSConvertContext, convert::TSConvert, resolve::TSConvertResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct TSConvertModule(pub TSModule);

impl TSConvertModule {
    pub fn convert(module: &GTModule, resolve: TSConvertResolve) -> Self {
        let mut context = TSConvertContext::new(resolve);

        let imports = module
            .imports
            .iter()
            .map(|import| import.convert(&mut context))
            .collect();

        let mut definitions = vec![];

        for alias in &module.aliases {
            let definition = alias.convert(&mut context);

            definitions.push(definition);
            definitions.extend(context.drain_hoisted());
        }

        let doc = module.doc.as_ref().map(|doc| {
            let mut doc = doc.convert(&mut context);
            doc.0 = "@file ".to_string() + &doc.0;
            doc
        });

        TSConvertModule(TSModule {
            doc,
            imports,
            definitions,
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut resolve = TSConvertResolve::new();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_eq!(
            TSConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: None,
                    imports: vec![
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GTImportReference::Glob((0, 0).into())
                        },
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GTImportReference::Names(
                                (0, 0).into(),
                                vec![
                                    GTImportName::Name(
                                        (0, 0).into(),
                                        GTIdentifier::new((0, 0).into(), "Name".into())
                                    ),
                                    GTImportName::Alias(
                                        (0, 0).into(),
                                        GTIdentifier::new((0, 0).into(), "Name".into()),
                                        GTIdentifier::new((0, 0).into(), "Alias".into())
                                    )
                                ]
                            )
                        }
                    ],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "User".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                name: GTIdentifier::new((0, 0).into(), "User".into()).into(),
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
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Order".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                name: GTIdentifier::new((0, 0).into(), "Order".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "book".into()),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        id: GTDefinitionId("module".into(), "Book".into()),
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTIdentifier::new((0, 0).into(), "Book".into()),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            span: (0, 0).into(),
                                            name: GTIdentifier::new((0, 0).into(), "Book".into())
                                                .into(),
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new((0, 0).into(), "title".into()),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((0, 0).into())
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (0, 0).into(),
                                                        "author".into()
                                                    ),
                                                    descriptor: GTReference {
                                                        span: (0, 0).into(),
                                                        id: GTReferenceId(
                                                            "module".into(),
                                                            (0, 0).into()
                                                        ),
                                                        definition_id:
                                                            GTReferenceDefinitionId::Resolved(
                                                                GTDefinitionId(
                                                                    "module".into(),
                                                                    "Author".into()
                                                                )
                                                            ),
                                                        identifier: GTIdentifier::new(
                                                            (0, 0).into(),
                                                            "Author".into()
                                                        )
                                                    }
                                                    .into(),
                                                    required: true,
                                                }
                                            ]
                                        })
                                    })),
                                    required: true,
                                },]
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Name".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                        },
                    ],
                },
                resolve
            ),
            TSConvertModule(TSModule {
                doc: None,
                imports: vec![
                    TSImport {
                        path: "./path/to/module.ts".into(),
                        reference: TSImportReference::Glob("module".into())
                    },
                    TSImport {
                        path: "./path/to/module.ts".into(),
                        reference: TSImportReference::Named(vec![
                            TSImportName::Name("Name".into()),
                            TSImportName::Alias("Name".into(), "Alias".into())
                        ])
                    }
                ],
                definitions: vec![
                    TSDefinition::Interface(TSInterface {
                        doc: None,
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            TSProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
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
                    }),
                    TSDefinition::Interface(TSInterface {
                        doc: None,
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![TSProperty {
                            doc: None,
                            name: "book".into(),
                            descriptor: TSDescriptor::Reference("Book".into()),
                            required: true,
                        }]
                    }),
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
                                descriptor: TSDescriptor::Reference("Author".into()),
                                required: true,
                            }
                        ]
                    }),
                    TSDefinition::Alias(TSAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_eq!(
            TSConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                TSConvertResolve::new()
            ),
            TSConvertModule(TSModule {
                doc: Some(TSDoc("@file Hello, world!".into())),
                imports: vec![],
                definitions: vec![]
            })
        );
    }
}
