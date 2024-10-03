use std::sync::Mutex;

use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::tree::module::GTModule;

use crate::convert::TSConvert;

pub fn convert_to_ts_module(module: &GTModule) -> TSModule {
    let imports = module
        .imports
        .iter()
        .map(|import| import.convert(&|_| {}))
        .collect();

    let definitions = Mutex::new(Vec::new());

    for alias in &module.aliases {
        let hoisted = Mutex::new(Vec::new());

        let definition = alias.convert(&|definition| {
            let mut hoisted = hoisted.lock().unwrap();
            hoisted.push(definition);
        });

        let mut definitions = definitions.lock().unwrap();
        definitions.push(definition);
        definitions.extend(hoisted.into_inner().unwrap());
    }

    TSModule {
        path: module.path.convert(&|_| {}),
        doc: None,
        imports,
        definitions: definitions.into_inner().unwrap(),
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
        assert_eq!(
            convert_to_ts_module(&GTModule {
                path: "./path/to/module".into(),
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
                            properties: vec![GTProperty {
                                doc: None,
                                name: "book".into(),
                                descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                    doc: None,
                                    name: "Book".into(),
                                    descriptor: GTDescriptor::Object(GTObject {
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
                                                    GTReference::External(
                                                        "Author".into(),
                                                        "./path/to/module".into()
                                                    )
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
            }),
            TSModule {
                path: TSPath::Unresolved("./path/to/module".into()),
                doc: None,
                imports: vec![
                    TSImport {
                        path: TSPath::Unresolved("./path/to/module".into()),
                        reference: TSImportReference::Glob(TSImportGlobAlias::Unresolved)
                    },
                    TSImport {
                        path: TSPath::Unresolved("./path/to/module".into()),
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
                            descriptor: TSDescriptor::Reference(TSReference::Local("Book".into(),)),
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
                                descriptor: TSDescriptor::Reference(TSReference::External(
                                    "Author".into(),
                                    TSPath::Unresolved("./path/to/module".into())
                                )),
                                required: true,
                            }
                        ]
                    }),
                    TSDefinition::Alias(TSAlias {
                        name: "Name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    }),
                ]
            },
        );
    }
}
