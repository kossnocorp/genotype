use crate::{
    parser::{parse_gt_code, Rule},
    tree::{GTAlias, GTDoc, GTImport, GTPath, GTResolve},
};

use super::GTModule;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModuleParse {
    pub module: GTModule,
    pub resolve: GTResolve,
}

impl GTModule {
    pub fn parse(path: GTPath, code: String) -> Result<GTModuleParse, Box<dyn std::error::Error>> {
        let mut pairs = parse_gt_code(&code)?;
        let mut module = GTModule {
            path,
            doc: None,
            imports: vec![],
            aliases: vec![],
        };
        let mut resolve = GTResolve::new();

        let module_pair = pairs.next().unwrap();
        for pair in module_pair.into_inner() {
            match pair.as_rule() {
                Rule::module_doc => {
                    let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                    if let Some(pair) = doc {
                        module.doc = Some(if let Some(doc) = module.doc {
                            doc.concat(pair)
                        } else {
                            GTDoc::parse(pair)
                        });
                    }
                }

                Rule::import => {
                    module.imports.push(GTImport::parse(pair, &mut resolve)?);
                }

                Rule::alias => {
                    module.aliases.push(GTAlias::parse(pair, &mut resolve)?);
                }

                Rule::EOI => {}

                _ => {
                    println!("1 ====== unknown rule: {:?}", pair);
                    unreachable!("unknown rule");
                }
            }
        }

        Ok(GTModuleParse { module, resolve })
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::*;
    use pretty_assertions::assert_eq;
    use std::{collections::HashSet, fs};

    use super::GTModuleParse;

    #[test]
    fn test_alias() {
        assert_module(
            "./examples/syntax/01-alias.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/01-alias.type".into(),
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
                            descriptor: GTDescriptor::Reference(GTReference::Unresolved(
                                "Age".into(),
                            )),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Age".into(), "AnotherAge".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_primitives() {
        assert_module(
            "./examples/syntax/02-primitives.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/02-primitives.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        "String".into(),
                        "Int".into(),
                        "Float".into(),
                        "Boolean".into(),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_objects() {
        assert_module(
            "./examples/syntax/03-objects.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/03-objects.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        "Hello".into(),
                        "Hello".into(),
                        "Empty".into(),
                        "Empty".into(),
                        "Hello".into(),
                        "Hello".into(),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_comments() {
        assert_module(
            "./examples/syntax/04-comments.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/04-comments.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Hello".into(), "Hello".into(), "Hello".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_optional() {
        assert_module(
            "./examples/syntax/05-optional.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/05-optional.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Hello".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_nested() {
        assert_module(
            "./examples/syntax/06-nested.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/06-nested.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Hello".into(), "Hello".into(), "Named".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_arrays() {
        assert_module(
            "./examples/syntax/07-arrays.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/07-arrays.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Book".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_tuples() {
        assert_module(
            "./examples/syntax/08-tuples.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/08-tuples.type".into(),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["User".into(), "Address".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_modules() {
        assert_module(
            "./examples/syntax/09-modules.type",
            GTModuleParse {
                module: GTModule {
                    path: "./examples/syntax/09-modules.type".into(),
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
                                        descriptor: GTDescriptor::Reference(
                                            GTReference::Unresolved("Genre".into()),
                                        ),
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
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec!["Book".into(), "Author".into()],
                    references: HashSet::new(),
                },
            },
        );
    }

    fn assert_module(path: &str, expected: GTModuleParse) {
        let code = fs::read_to_string(path).expect("cannot read file");
        let parse = GTModule::parse(path.into(), code).unwrap();
        assert_eq!(parse, expected);
    }
}
