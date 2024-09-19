use crate::parser::{self, Rule};
use pest::iterators::Pairs;

use super::alias::{parse_alias, Alias};

#[derive(Debug, PartialEq)]
pub struct Module {
    doc: Option<String>,
    aliases: Vec<Alias>,
}

pub fn parse_module(mut pairs: Pairs<'_, Rule>) -> Result<Module, Box<dyn std::error::Error>> {
    let mut module = Module {
        doc: None,
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

            Rule::alias => {
                let (alias, hoisted) = parse_alias(pair)?;
                module.aliases.push(alias);
                module.aliases.extend(hoisted);
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

enum ParseState {
    Doc,
    Module(Option<String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{
        descriptor::Descriptor, object::Object, primitive::Primitive, property::Property,
    };
    use parser::parse_code;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_alias() {
        assert_module(
            "../examples/syntax/01-alias.type",
            Module {
                doc: None,
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
            "../examples/syntax/02-primitives.type",
            Module {
                doc: None,
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
            "../examples/syntax/03-objects.type",
            Module {
                doc: None,
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
            "../examples/syntax/04-comments.type",
            Module {
                doc: Some("Module comment...\n...multiline".to_string()),
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
            "../examples/syntax/05-optional.type",
            Module {
                doc: None,
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
            "../examples/syntax/06-nested.type",
            Module {
                doc: None,
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
                                descriptor: Descriptor::Name("Named".to_string()),
                                required: true,
                            }],
                        }),
                    },
                    Alias {
                        doc: None,
                        name: "Named".to_string(),
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
                let module = parse_module(pairs);
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
