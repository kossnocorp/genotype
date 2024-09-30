use super::{
    alias::{parse_alias, GTAlias},
    import::GTImport,
};
use crate::{parser::Rule, tree::import::parse_import};
use pest::iterators::Pairs;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub path: String,
    pub doc: Option<String>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}

pub fn parse_gt_module(
    path: String,
    mut pairs: Pairs<'_, Rule>,
) -> Result<GTModule, Box<dyn std::error::Error>> {
    let mut module = GTModule {
        path,
        doc: None,
        imports: vec![],
        aliases: vec![],
    };

    let module_pair = pairs.next().unwrap();

    for pair in module_pair.into_inner() {
        match pair.as_rule() {
            Rule::module_doc => {
                let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                if let Some(pair) = doc {
                    module.doc = Some(if let Some(str) = module.doc {
                        str + "\n" + pair.as_str()
                    } else {
                        pair.as_str().to_string()
                    });
                }
            }

            Rule::import => {
                // [TODO]
                let import = parse_import(pair)?;
                module.imports.push(import);
            }

            Rule::alias => {
                let alias = parse_alias(pair)?;
                module.aliases.push(alias);
            }

            Rule::EOI => {}

            _ => {
                println!("1 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        }
    }

    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parser::parse_gt_code,
        tree::{
            array::GTArray,
            descriptor::GTDescriptor,
            import::{ImportName, ImportReference},
            name::GTName,
            object::GTObject,
            primitive::GTPrimitive,
            property::GTProperty,
            reference::GTReference,
            tuple::GTTuple,
        },
    };
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_alias() {
        assert_module(
            "./examples/syntax/01-alias.type",
            GTModule {
                path: "./examples/syntax/01-alias.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("Age".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("AnotherAge".to_string()),
                        descriptor: GTDescriptor::Name(GTName("Age".to_string())),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_primitives() {
        assert_module(
            "./examples/syntax/02-primitives.type",
            GTModule {
                path: "./examples/syntax/02-primitives.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("String".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Int".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Float".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Float),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Boolean".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_objects() {
        assert_module(
            "./examples/syntax/03-objects.type",
            GTModule {
                path: "./examples/syntax/03-objects.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: GTName("name".to_string()),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            }],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
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
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: GTName("flag".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Empty".to_string()),
                        descriptor: GTDescriptor::Object(GTObject { properties: vec![] }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Empty".to_string()),
                        descriptor: GTDescriptor::Object(GTObject { properties: vec![] }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: GTName("name".to_string()),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            }],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
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
                                    required: true,
                                },
                            ],
                        }),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_comments() {
        assert_module(
            "./examples/syntax/04-comments.type",
            GTModule {
                path: "./examples/syntax/04-comments.type".to_string(),
                doc: Some("Module comment...\n...multiline".to_string()),
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: Some("Alias comment".to_string()),
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                    GTAlias {
                        doc: Some("Multiline...\n...alias comment".to_string()),
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: Some("Property comment".to_string()),
                                    name: GTName("name".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: Some("Multiline...\n...property comment".to_string()),
                                    name: GTName("age".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_optional() {
        assert_module(
            "./examples/syntax/05-optional.type",
            GTModule {
                path: "./examples/syntax/05-optional.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![GTAlias {
                    doc: None,
                    name: GTName("Hello".to_string()),
                    descriptor: GTDescriptor::Object(GTObject {
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: GTName("name".to_string()),
                                descriptor: GTDescriptor::Nullable(Box::new(
                                    GTDescriptor::Primitive(GTPrimitive::String),
                                )),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: GTName("age".to_string()),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                required: false,
                            },
                            GTProperty {
                                doc: None,
                                name: GTName("flag".to_string()),
                                descriptor: GTDescriptor::Nullable(Box::new(
                                    GTDescriptor::Primitive(GTPrimitive::Boolean),
                                )),
                                required: false,
                            },
                        ],
                    }),
                }],
            },
        );
    }

    #[test]
    fn test_nested() {
        assert_module(
            "./examples/syntax/06-nested.type",
            GTModule {
                path: "./examples/syntax/06-nested.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: GTName("name".to_string()),
                                descriptor: GTDescriptor::Object(GTObject {
                                    properties: vec![
                                        GTProperty {
                                            doc: None,
                                            name: GTName("first".to_string()),
                                            descriptor: GTDescriptor::Primitive(
                                                GTPrimitive::String,
                                            ),
                                            required: true,
                                        },
                                        GTProperty {
                                            doc: None,
                                            name: GTName("last".to_string()),
                                            descriptor: GTDescriptor::Primitive(
                                                GTPrimitive::String,
                                            ),
                                            required: true,
                                        },
                                    ],
                                }),
                                required: true,
                            }],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Hello".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: GTName("name".to_string()),
                                descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                    doc: None,
                                    name: GTName("Named".to_string()),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        properties: vec![
                                            GTProperty {
                                                doc: None,
                                                name: GTName("first".to_string()),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String,
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                doc: None,
                                                name: GTName("last".to_string()),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String,
                                                ),
                                                required: true,
                                            },
                                        ],
                                    }),
                                })),
                                required: true,
                            }],
                        }),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_arrays() {
        assert_module(
            "./examples/syntax/07-arrays.type",
            GTModule {
                path: "./examples/syntax/07-arrays.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![GTAlias {
                    doc: None,
                    name: GTName("Book".to_string()),
                    descriptor: GTDescriptor::Object(GTObject {
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: GTName("title".to_string()),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: GTName("tags".to_string()),
                                descriptor: GTDescriptor::Array(Box::new(GTArray {
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                })),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
        );
    }

    #[test]
    fn test_tuples() {
        assert_module(
            "./examples/syntax/08-tuples.type",
            GTModule {
                path: "./examples/syntax/08-tuples.type".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("User".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: GTName("name".to_string()),
                                    descriptor: GTDescriptor::Tuple(GTTuple {
                                        descriptors: vec![
                                            GTDescriptor::Primitive(GTPrimitive::String),
                                            GTDescriptor::Primitive(GTPrimitive::String),
                                        ],
                                    }),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: GTName("address".to_string()),
                                    descriptor: GTDescriptor::Tuple(GTTuple {
                                        descriptors: vec![
                                            GTDescriptor::Primitive(GTPrimitive::Int),
                                            GTDescriptor::Primitive(GTPrimitive::String),
                                            GTDescriptor::Primitive(GTPrimitive::String),
                                        ],
                                    }),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Address".to_string()),
                        descriptor: GTDescriptor::Tuple(GTTuple {
                            descriptors: vec![
                                GTDescriptor::Primitive(GTPrimitive::Int),
                                GTDescriptor::Primitive(GTPrimitive::String),
                                GTDescriptor::Primitive(GTPrimitive::String),
                            ],
                        }),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_modules() {
        assert_module(
            "./examples/syntax/09-modules.type",
            GTModule {
                path: "./examples/syntax/09-modules.type".to_string(),
                doc: None,
                imports: vec![
                    GTImport {
                        path: "author".to_string(),
                        reference: ImportReference::Glob,
                    },
                    GTImport {
                        path: "../../author".to_string(),
                        reference: ImportReference::Names(vec![
                            ImportName::Name("Author".to_string()),
                            ImportName::Name("Genre".to_string()),
                            ImportName::Alias("Something".to_string(), "Else".to_string()),
                        ]),
                    },
                    GTImport {
                        path: "author".to_string(),
                        reference: ImportReference::Name("Author".to_string()),
                    },
                ],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: GTName("Book".to_string()),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: GTName("title".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: GTName("author".to_string()),
                                    descriptor: GTDescriptor::Reference(GTReference {
                                        path: "../../author".to_string(),
                                        name: GTName("Author".to_string()),
                                    }),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: GTName("genre".to_string()),
                                    descriptor: GTDescriptor::Name(GTName("Genre".to_string())),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: GTName("Author".to_string()),
                        descriptor: GTDescriptor::Reference(GTReference {
                            path: "../../author".to_string(),
                            name: GTName("Author".to_string()),
                        }),
                    },
                ],
            },
        );
    }

    fn assert_module(path: &str, expected: GTModule) {
        let code = fs::read_to_string(path).expect("cannot read file");
        let pairs = parse_gt_code(&code);

        match pairs {
            Ok(pairs) => {
                let module = parse_gt_module(path.to_string(), pairs);
                match module {
                    Ok(module) => {
                        assert_eq!(module, expected);
                    }

                    Err(err) => {
                        println!("{}", err);
                        assert!(false, "Failed to build module");
                    }
                }
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to parse file");
            }
        }
    }
}
