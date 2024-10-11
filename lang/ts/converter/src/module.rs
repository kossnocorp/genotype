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
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut resolve = TSConvertResolve::new();
        resolve
            .globs
            .insert("./path/to/module".into(), "module".into());

        assert_eq!(
            TSConvertModule::convert(
                &GTModule {
                    doc: None,
                    imports: vec![
                        GTImport {
                            path: "./path/to/module".into(),
                            reference: GTImportReference::Glob
                        },
                        GTImport {
                            path: "./path/to/module".into(),
                            reference: GTImportReference::Names(vec![
                                GTImportName::Name("Name".into()),
                                GTImportName::Alias("Name".into(), "Alias".into())
                            ])
                        }
                    ],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: "User".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String.into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "age".into(),
                                        descriptor: GTPrimitive::Int.into(),
                                        required: false,
                                    }
                                ]
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: "Order".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "book".into(),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        doc: None,
                                        name: "Book".into(),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    doc: None,
                                                    name: "title".into(),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    doc: None,
                                                    name: "author".into(),
                                                    descriptor: GTDescriptor::Reference(
                                                        "Author".into()
                                                    ),
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
                            name: "Name".into(),
                            descriptor: GTPrimitive::String.into(),
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
                        properties: vec![TSProperty {
                            name: "book".into(),
                            descriptor: TSDescriptor::Reference("Book".into()),
                            required: true,
                        }]
                    }),
                    TSDefinition::Interface(TSInterface {
                        name: "Book".into(),
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
