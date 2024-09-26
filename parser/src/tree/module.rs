use std::path::PathBuf;

use super::{
    alias::{parse_alias, Alias},
    import::Import,
};
use crate::{parser::Rule, tree::import::parse_import};
use pest::iterators::Pairs;

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub path: PathBuf,
    pub doc: Option<String>,
    pub imports: Vec<Import>,
    pub aliases: Vec<Alias>,
}

pub fn parse_module(
    path: PathBuf,
    mut pairs: Pairs<'_, Rule>,
) -> Result<Module, Box<dyn std::error::Error>> {
    let path = path.canonicalize()?;

    let mut module = Module {
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
        parser::parse_code,
        tree::{
            array::Array,
            descriptor::Descriptor,
            import::{ImportName, ImportReference},
            object::Object,
            primitive::Primitive,
            property::Property,
            reference::Reference,
            tuple::Tuple,
        },
    };
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_alias() {
        assert_module(
            "./examples/syntax/01-alias.type",
            Module {
                path: PathBuf::from("./examples/syntax/01-alias.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "Age".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::Int),
                    },
                    Alias {
                        doc: None,
                        name: "AnotherAge".to_string(),
                        descriptor: Descriptor::Name("Age".to_string()),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_primitives() {
        assert_module(
            "./examples/syntax/02-primitives.type",
            Module {
                path: PathBuf::from("./examples/syntax/02-primitives.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "String".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::String),
                    },
                    Alias {
                        doc: None,
                        name: "Int".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::Int),
                    },
                    Alias {
                        doc: None,
                        name: "Float".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::Float),
                    },
                    Alias {
                        doc: None,
                        name: "Boolean".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::Boolean),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_objects() {
        assert_module(
            "./examples/syntax/03-objects.type",
            Module {
                path: PathBuf::from("./examples/syntax/03-objects.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            }],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![
                                Property {
                                    doc: None,
                                    name: "name".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::String),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "age".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::Int),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "flag".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::Boolean),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Empty".to_string(),
                        descriptor: Descriptor::Object(Object { properties: vec![] }),
                    },
                    Alias {
                        doc: None,
                        name: "Empty".to_string(),
                        descriptor: Descriptor::Object(Object { properties: vec![] }),
                    },
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            }],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![
                                Property {
                                    doc: None,
                                    name: "name".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::String),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "age".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::Int),
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
            Module {
                path: PathBuf::from("./examples/syntax/04-comments.type")
                    .canonicalize()
                    .unwrap(),
                doc: Some("Module comment...\n...multiline".to_string()),
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: Some("Alias comment".to_string()),
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::String),
                    },
                    Alias {
                        doc: Some("Multiline...\n...alias comment".to_string()),
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![
                                Property {
                                    doc: Some("Property comment".to_string()),
                                    name: "name".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::String),
                                    required: true,
                                },
                                Property {
                                    doc: Some("Multiline...\n...property comment".to_string()),
                                    name: "age".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::Int),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Primitive(Primitive::String),
                    },
                ],
            },
        );
    }

    #[test]
    fn test_optional() {
        assert_module(
            "./examples/syntax/05-optional.type",
            Module {
                path: PathBuf::from("./examples/syntax/05-optional.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![Alias {
                    doc: None,
                    name: "Hello".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![
                            Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Nullable(Box::new(Descriptor::Primitive(
                                    Primitive::String,
                                ))),
                                required: true,
                            },
                            Property {
                                doc: None,
                                name: "age".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::Int),
                                required: false,
                            },
                            Property {
                                doc: None,
                                name: "flag".to_string(),
                                descriptor: Descriptor::Nullable(Box::new(Descriptor::Primitive(
                                    Primitive::Boolean,
                                ))),
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
            Module {
                path: PathBuf::from("./examples/syntax/06-nested.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Object(Object {
                                    properties: vec![
                                        Property {
                                            doc: None,
                                            name: "first".to_string(),
                                            descriptor: Descriptor::Primitive(Primitive::String),
                                            required: true,
                                        },
                                        Property {
                                            doc: None,
                                            name: "last".to_string(),
                                            descriptor: Descriptor::Primitive(Primitive::String),
                                            required: true,
                                        },
                                    ],
                                }),
                                required: true,
                            }],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Hello".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Alias(Box::new(Alias {
                                    doc: None,
                                    name: "Named".to_string(),
                                    descriptor: Descriptor::Object(Object {
                                        properties: vec![
                                            Property {
                                                doc: None,
                                                name: "first".to_string(),
                                                descriptor: Descriptor::Primitive(
                                                    Primitive::String,
                                                ),
                                                required: true,
                                            },
                                            Property {
                                                doc: None,
                                                name: "last".to_string(),
                                                descriptor: Descriptor::Primitive(
                                                    Primitive::String,
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
            Module {
                path: PathBuf::from("./examples/syntax/07-arrays.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![Alias {
                    doc: None,
                    name: "Book".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![
                            Property {
                                doc: None,
                                name: "title".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            },
                            Property {
                                doc: None,
                                name: "tags".to_string(),
                                descriptor: Descriptor::Array(Box::new(Array {
                                    descriptor: Descriptor::Primitive(Primitive::String),
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
            Module {
                path: PathBuf::from("./examples/syntax/08-tuples.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "User".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![
                                Property {
                                    doc: None,
                                    name: "name".to_string(),
                                    descriptor: Descriptor::Tuple(Tuple {
                                        descriptors: vec![
                                            Descriptor::Primitive(Primitive::String),
                                            Descriptor::Primitive(Primitive::String),
                                        ],
                                    }),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "address".to_string(),
                                    descriptor: Descriptor::Tuple(Tuple {
                                        descriptors: vec![
                                            Descriptor::Primitive(Primitive::Int),
                                            Descriptor::Primitive(Primitive::String),
                                            Descriptor::Primitive(Primitive::String),
                                        ],
                                    }),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Address".to_string(),
                        descriptor: Descriptor::Tuple(Tuple {
                            descriptors: vec![
                                Descriptor::Primitive(Primitive::Int),
                                Descriptor::Primitive(Primitive::String),
                                Descriptor::Primitive(Primitive::String),
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
            Module {
                path: PathBuf::from("./examples/syntax/09-modules.type")
                    .canonicalize()
                    .unwrap(),
                doc: None,
                imports: vec![
                    Import {
                        path: "author/".to_string(),
                        reference: ImportReference::Glob,
                    },
                    Import {
                        path: "../../author/".to_string(),
                        reference: ImportReference::Names(vec![
                            ImportName::Name("Author".to_string()),
                            ImportName::Name("Genre".to_string()),
                            ImportName::Alias("Something".to_string(), "Else".to_string()),
                        ]),
                    },
                    Import {
                        path: "author/".to_string(),
                        reference: ImportReference::Name("Author".to_string()),
                    },
                ],
                aliases: vec![
                    Alias {
                        doc: None,
                        name: "Book".to_string(),
                        descriptor: Descriptor::Object(Object {
                            properties: vec![
                                Property {
                                    doc: None,
                                    name: "title".to_string(),
                                    descriptor: Descriptor::Primitive(Primitive::String),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "author".to_string(),
                                    descriptor: Descriptor::Reference(Reference {
                                        path: "../../author".to_string(),
                                        name: "Author".to_string(),
                                    }),
                                    required: true,
                                },
                                Property {
                                    doc: None,
                                    name: "genre".to_string(),
                                    descriptor: Descriptor::Name("Genre".to_string()),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Author".to_string(),
                        descriptor: Descriptor::Reference(Reference {
                            path: "../../author".to_string(),
                            name: "Author".to_string(),
                        }),
                    },
                ],
            },
        );
    }

    fn assert_module(file: &str, expected: Module) {
        let code = fs::read_to_string(file).expect("cannot read file");
        let pairs = parse_code(&code);

        match pairs {
            Ok(pairs) => {
                let module = parse_module(PathBuf::from(file), pairs);
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
