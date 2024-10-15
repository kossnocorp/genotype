use std::sync::Mutex;

use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::tree::module::GTModule;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct TSConvertModule(pub TSModule);

impl TSConvertModule {
    pub fn convert(module: &GTModule, resolve: &TSConvertResolve) -> Self {
        let imports = module
            .imports
            .iter()
            .map(|import| import.convert(resolve, &|_| {}))
            .collect();

        let definitions = Mutex::new(Vec::new());

        for alias in &module.aliases {
            let hoisted = Mutex::new(Vec::new());

            let definition = alias.convert(resolve, &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            });

            let mut definitions = definitions.lock().unwrap();
            definitions.push(definition);
            definitions.extend(hoisted.into_inner().unwrap());
        }

        TSConvertModule(TSModule {
            doc: None,
            imports,
            definitions: definitions.into_inner().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::{tree::*, GTSourceCode};
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
                    source_code: GTSourceCode::new("module.type".into(), "".into()),
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
                            doc: None,
                            name: GTIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        name: GTKey::new((0, 0).into(), "name".into()),
                                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        name: GTKey::new((0, 0).into(), "age".into()),
                                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                                        required: false,
                                    }
                                ]
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    name: GTKey::new((0, 0).into(), "book".into()),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        doc: None,
                                        name: GTIdentifier::new((0, 0).into(), "Book".into()),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            span: (0, 0).into(),
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    name: GTKey::new((0, 0).into(), "title".into()),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((0, 0).into())
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    name: GTKey::new(
                                                        (0, 0).into(),
                                                        "author".into()
                                                    ),
                                                    descriptor: GTIdentifier::new(
                                                        (0, 0).into(),
                                                        "Author".into()
                                                    )
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
                            doc: None,
                            name: GTIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                        },
                    ],
                },
                &resolve
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
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            TSProperty {
                                name: "name".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            },
                            TSProperty {
                                name: "age".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                                required: false,
                            }
                        ]
                    }),
                    TSDefinition::Interface(TSInterface {
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![TSProperty {
                            name: "book".into(),
                            descriptor: TSDescriptor::Reference("Book".into()),
                            required: true,
                        }]
                    }),
                    TSDefinition::Interface(TSInterface {
                        name: "Book".into(),
                        extensions: vec![],
                        properties: vec![
                            TSProperty {
                                name: "title".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            },
                            TSProperty {
                                name: "author".into(),
                                descriptor: TSDescriptor::Reference("Author".into()),
                                required: true,
                            }
                        ]
                    }),
                    TSDefinition::Alias(TSAlias {
                        name: "Name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    }),
                ]
            })
        );
    }
}
