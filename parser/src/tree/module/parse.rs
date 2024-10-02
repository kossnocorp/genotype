use pest::iterators::Pairs;

use crate::parser::{parse_gt_code, Rule};

use super::GTModule;

impl GTModule {
    pub fn parse(code: String) -> Result<Self, Box<dyn std::error::Error>> {
        let pairs = parse_gt_code(&code)?;
        pairs.try_into()
    }
}

impl TryFrom<Pairs<'_, Rule>> for GTModule {
    type Error = Box<dyn std::error::Error>;

    fn try_from(mut pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let mut module = GTModule {
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
                        module.doc = Some(if let Some(doc) = module.doc {
                            doc.concat(pair)
                        } else {
                            pair.into()
                        });
                    }
                }

                Rule::import => {
                    module.imports.push(pair.try_into()?);
                }

                Rule::alias => {
                    module.aliases.push(pair.try_into()?);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parser::parse_gt_code,
        tree::{
            alias::GTAlias, array::GTArray, descriptor::GTDescriptor, import::GTImport,
            import_name::GTImportName, import_reference::GTImportReference,
            inline_import::GTInlineImport, object::GTObject, path::GTPath, primitive::GTPrimitive,
            property::GTProperty, reference::GTReference, tuple::GTTuple,
        },
    };
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_alias() {
        assert_module(
            "./examples/syntax/01-alias.type",
            GTModule {
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "Age".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                    },
                    GTAlias {
                        doc: None,
                        name: "AnotherAge".into(),
                        descriptor: GTDescriptor::Reference(GTReference::Unresolved("Age".into())),
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
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "String".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                    GTAlias {
                        doc: None,
                        name: "Int".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                    },
                    GTAlias {
                        doc: None,
                        name: "Float".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Float),
                    },
                    GTAlias {
                        doc: None,
                        name: "Boolean".into(),
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
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            }],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "age".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "flag".into(),
                                    descriptor: GTDescriptor::Primitive(
                                        GTPrimitive::Boolean.into(),
                                    ),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Empty".into(),
                        descriptor: GTDescriptor::Object(GTObject { properties: vec![] }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Empty".into(),
                        descriptor: GTDescriptor::Object(GTObject { properties: vec![] }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            }],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "age".into(),
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
                doc: Some("Module comment...\n...multiline".into()),
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: Some("Alias comment".into()),
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                    },
                    GTAlias {
                        doc: Some("Multiline...\n...alias comment".into()),
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: Some("Property comment".into()),
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: Some("Multiline...\n...property comment".into()),
                                    name: "age".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
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
                doc: None,
                imports: vec![],
                aliases: vec![GTAlias {
                    doc: None,
                    name: "Hello".into(),
                    descriptor: GTDescriptor::Object(GTObject {
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: GTDescriptor::Nullable(Box::new(
                                    GTDescriptor::Primitive(GTPrimitive::String),
                                )),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                                required: false,
                            },
                            GTProperty {
                                doc: None,
                                name: "flag".into(),
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
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: GTDescriptor::Object(GTObject {
                                    properties: vec![
                                        GTProperty {
                                            doc: None,
                                            name: "first".into(),
                                            descriptor: GTDescriptor::Primitive(
                                                GTPrimitive::String,
                                            ),
                                            required: true,
                                        },
                                        GTProperty {
                                            doc: None,
                                            name: "last".into(),
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
                        name: "Hello".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![GTProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                    doc: None,
                                    name: "Named".into(),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        properties: vec![
                                            GTProperty {
                                                doc: None,
                                                name: "first".into(),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String,
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                doc: None,
                                                name: "last".into(),
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
                doc: None,
                imports: vec![],
                aliases: vec![GTAlias {
                    doc: None,
                    name: "Book".into(),
                    descriptor: GTDescriptor::Object(GTObject {
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: "title".into(),
                                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: "tags".into(),
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
                doc: None,
                imports: vec![],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "User".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "name".into(),
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
                                    name: "address".into(),
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
                        name: "Address".into(),
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
                doc: None,
                imports: vec![
                    GTImport {
                        path: GTPath("author".into()),
                        reference: GTImportReference::Glob,
                    },
                    GTImport {
                        path: GTPath("../../author".into()),
                        reference: GTImportReference::Names(vec![
                            GTImportName::Name("Author".into()),
                            GTImportName::Name("Genre".into()),
                            GTImportName::Alias("Something".into(), "Else".into()),
                        ]),
                    },
                    GTImport {
                        path: GTPath("author".into()),
                        reference: GTImportReference::Name("Author".into()),
                    },
                ],
                aliases: vec![
                    GTAlias {
                        doc: None,
                        name: "Book".into(),
                        descriptor: GTDescriptor::Object(GTObject {
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "author".into(),
                                    descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                        path: GTPath("../../author".into()),
                                        name: "Author".into(),
                                    }),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "genre".into(),
                                    descriptor: GTDescriptor::Reference(GTReference::Unresolved(
                                        "Genre".into(),
                                    )),
                                    required: true,
                                },
                            ],
                        }),
                    },
                    GTAlias {
                        doc: None,
                        name: "Author".into(),
                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                            path: GTPath("../../author".into()),
                            name: "Author".into(),
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
                let module = TryInto::<GTModule>::try_into(pairs);
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
