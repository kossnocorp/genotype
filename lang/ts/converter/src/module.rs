use std::sync::Mutex;

use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::tree::module::GTModule;

use crate::convert::TSConvert;

pub fn convert_to_ts_module(module: GTModule) -> TSModule {
    let imports = module
        .imports
        .iter()
        .map(|import| import.convert(&|_| {}))
        .collect();

    let definitions = Mutex::new(Vec::new());

    for alias in module.aliases {
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
        doc: None,
        imports,
        definitions: definitions.into_inner().unwrap(),
    }
}

#[cfg(test)]
mod tests {

    use genotype_lang_ts_tree::{
        alias::TSAlias, definition::TSDefinition, import::TSImport,
        import_glob_alias::TSImportGlobAlias, import_name::TSImportName,
        import_reference::TSImportReference, interface::TSInterface, name::TSName,
        primitive::TSPrimitive, property::TSProperty, type_descriptor::TSTypeDescriptor,
    };
    use genotype_parser::tree::{
        alias::GTAlias, descriptor::GTDescriptor, import::GTImport, import_name::GTImportName,
        import_reference::GTImportReference, name::GTName, object::GTObject,
        primitive::GTPrimitive, property::GTProperty,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            convert_to_ts_module(GTModule {
                doc: None,
                imports: vec![
                    GTImport {
                        path: "./path/to/module".to_string(),
                        reference: GTImportReference::Glob
                    },
                    GTImport {
                        path: "./path/to/module".to_string(),
                        reference: GTImportReference::Names(vec![
                            GTImportName::Name(GTName("Name".to_string())),
                            GTImportName::Alias(
                                GTName("Name".to_string()),
                                GTName("Alias".to_string())
                            )
                        ])
                    }
                ],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("User".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: GTName("name".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: GTName("age".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                    required: false,
                                }
                            ]
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Order".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: GTName("book".to_string()),
                                descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                    doc: None,
                                    name: GTName("Book".to_string()),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        properties: vec![
                                            GTProperty {
                                                doc: None,
                                                name: GTName("title".to_string()),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                doc: None,
                                                name: GTName("author".to_string()),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String
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
                        name: GTName("Name".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                ],
            }),
            TSModule {
                doc: None,
                imports: vec![
                    TSImport {
                        path: "./path/to/module".to_string(),
                        reference: TSImportReference::Glob(TSImportGlobAlias::Unresolved)
                    },
                    TSImport {
                        path: "./path/to/module".to_string(),
                        reference: TSImportReference::Named(vec![
                            TSImportName::Name(TSName("Name".to_string())),
                            TSImportName::Alias(
                                TSName("Name".to_string()),
                                TSName("Alias".to_string())
                            )
                        ])
                    }
                ],
                definitions: vec![
                    TSDefinition::Interface(TSInterface {
                        name: TSName("User".to_string()),
                        properties: vec![
                            TSProperty {
                                name: TSName("name".to_string()),
                                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            },
                            TSProperty {
                                name: TSName("age".to_string()),
                                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                                required: false,
                            }
                        ]
                    }),
                    TSDefinition::Interface(TSInterface {
                        name: TSName("Order".to_string()),
                        properties: vec![TSProperty {
                            name: TSName("book".to_string()),
                            descriptor: TSTypeDescriptor::Name(TSName("Book".into())),
                            required: true,
                        }]
                    }),
                    TSDefinition::Interface(TSInterface {
                        name: TSName("Book".to_string()),
                        properties: vec![
                            TSProperty {
                                name: TSName("title".to_string()),
                                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            },
                            TSProperty {
                                name: TSName("author".to_string()),
                                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            }
                        ]
                    }),
                    TSDefinition::Alias(TSAlias {
                        name: TSName("Name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                    }),
                ]
            },
        );
    }
}
