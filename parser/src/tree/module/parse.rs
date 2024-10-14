use miette::Result;
use pest::iterators::Pairs;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModuleParse {
    pub module: GTModule,
    pub resolve: GTResolve,
}

impl GTModule {
    pub fn parse<'a>(source_code: GTSourceCode) -> Result<GTModuleParse> {
        match parse_gt_code(&source_code.content) {
            Ok(mut pairs) => match Self::parse_pairs(&mut pairs) {
                Ok(result) => Ok(GTModuleParse {
                    resolve: result.resolve,
                    module: GTModule {
                        source_code,
                        doc: result.doc,
                        imports: result.imports,
                        aliases: result.aliases,
                    },
                }),

                Err(error) => Err(GTModuleParseError::from_node_error(source_code, error).into()),
            },

            Err(error) => Err(GTModuleParseError::from_pest_error(source_code, error).into()),
        }
    }

    fn parse_pairs(pairs: &mut Pairs<'_, Rule>) -> Result<ModuleParseResult, GTNodeParseError> {
        let mut doc: Option<GTDoc> = None;
        let mut imports = vec![];
        let mut aliases = vec![];
        let mut resolve = GTResolve::new();

        let module_pair = pairs.next().unwrap();
        for pair in module_pair.into_inner() {
            match pair.as_rule() {
                Rule::module_doc => {
                    let doc_pair = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                    if let Some(pair) = doc_pair {
                        doc = Some(if let Some(doc_pair) = doc {
                            doc_pair.concat(pair)
                        } else {
                            GTDoc::parse(pair)
                        });
                    }
                }

                Rule::import => {
                    imports.push(GTImport::parse(pair, &mut resolve)?);
                }

                Rule::alias => {
                    aliases.push(GTAlias::parse(pair, &mut resolve)?);
                }

                Rule::EOI => {}

                _ => {
                    println!("1 ====== unknown rule: {:?}", pair);
                    unreachable!("unknown rule");
                }
            }
        }

        Ok(ModuleParseResult {
            doc,
            imports,
            aliases,
            resolve,
        })
    }
}

struct ModuleParseResult {
    doc: Option<GTDoc>,
    imports: Vec<GTImport>,
    aliases: Vec<GTAlias>,
    resolve: GTResolve,
}

#[cfg(test)]
mod tests {
    use crate::tree::*;
    use pretty_assertions::assert_eq;
    use std::{collections::HashSet, fs};

    use super::*;

    #[test]
    fn test_alias() {
        let source_code = read_source_code("./examples/syntax/01-alias.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 3).into(), "Age".into()),
                            descriptor: GTPrimitive::Int((6, 9).into()).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((11, 21).into(), "AnotherAge".into()),
                            descriptor: GTIdentifier::new((24, 27).into(), "Age".into()).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 3).into(), "Age".into()),
                        GTIdentifier::new((11, 21).into(), "AnotherAge".into()),
                    ],
                    references: HashSet::from_iter(vec![GTIdentifier::new(
                        (24, 27).into(),
                        "Age".into(),
                    )]),
                },
            },
        );
    }

    #[test]
    fn test_primitives() {
        let source_code = read_source_code("./examples/syntax/02-primitives.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 6).into(), "String".into()),
                            descriptor: GTPrimitive::String((9, 15).into()).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((17, 20).into(), "Int".into()),
                            descriptor: GTPrimitive::Int((23, 26).into()).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((28, 33).into(), "Float".into()),
                            descriptor: GTPrimitive::Float((36, 41).into()).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((43, 50).into(), "Boolean".into()),
                            descriptor: GTPrimitive::Boolean((53, 60).into()).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 6).into(), "String".into()),
                        GTIdentifier::new((17, 20).into(), "Int".into()),
                        GTIdentifier::new((28, 33).into(), "Float".into()),
                        GTIdentifier::new((43, 50).into(), "Boolean".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_objects() {
        let source_code = read_source_code("./examples/syntax/03-objects.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTPrimitive::String((18, 24).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((28, 33).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String((46, 52).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "age".into(),
                                        descriptor: GTPrimitive::Int((60, 63).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "flag".into(),
                                        descriptor: GTDescriptor::Primitive(
                                            GTPrimitive::Boolean((72, 79).into()).into(),
                                        ),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((83, 88).into(), "Empty".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((95, 100).into(), "Empty".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((111, 116).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTPrimitive::String((127, 133).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((137, 142).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String((153, 159).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "age".into(),
                                        descriptor: GTPrimitive::Int((166, 169).into()).into(),
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
                        GTIdentifier::new((0, 5).into(), "Hello".into()),
                        GTIdentifier::new((28, 33).into(), "Hello".into()),
                        GTIdentifier::new((83, 88).into(), "Empty".into()),
                        GTIdentifier::new((95, 100).into(), "Empty".into()),
                        GTIdentifier::new((111, 116).into(), "Hello".into()),
                        GTIdentifier::new((137, 142).into(), "Hello".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_comments() {
        let source_code = read_source_code("./examples/syntax/04-comments.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: Some("Module comment...\n...multiline".into()),
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: Some("Alias comment".into()),
                            name: GTIdentifier::new((76, 81).into(), "Hello".into()),
                            descriptor: GTPrimitive::String((105, 111).into()).into(),
                        },
                        GTAlias {
                            doc: Some("Multiline...\n...alias comment".into()),
                            name: GTIdentifier::new((151, 156).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: Some("Property comment".into()),
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String((192, 198).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: Some("Multiline...\n...property comment".into()),
                                        name: "age".into(),
                                        descriptor: GTPrimitive::Int((251, 254).into()).into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((258, 263).into(), "Hello".into()),
                            descriptor: GTPrimitive::String((266, 272).into()).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((76, 81).into(), "Hello".into()),
                        GTIdentifier::new((151, 156).into(), "Hello".into()),
                        GTIdentifier::new((258, 263).into(), "Hello".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_optional() {
        let source_code = read_source_code("./examples/syntax/05-optional.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![GTAlias {
                        doc: None,
                        name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                        descriptor: GTDescriptor::Object(GTObject {
                            extensions: vec![],
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Nullable(Box::new(
                                        GTPrimitive::String((18, 24).into()).into(),
                                    )),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "age".into(),
                                    descriptor: GTPrimitive::Int((34, 37).into()).into(),
                                    required: false,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "flag".into(),
                                    descriptor: GTDescriptor::Nullable(Box::new(
                                        GTPrimitive::Boolean((47, 54).into()).into(),
                                    )),
                                    required: false,
                                },
                            ],
                        }),
                    }],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![GTIdentifier::new((0, 5).into(), "Hello".into())],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_nested() {
        let source_code = read_source_code("./examples/syntax/06-nested.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        extensions: vec![],
                                        properties: vec![
                                            GTProperty {
                                                doc: None,
                                                name: "first".into(),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String((31, 37).into()),
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                doc: None,
                                                name: "last".into(),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String((48, 54).into()),
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
                            name: GTIdentifier::new((62, 67).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        doc: None,
                                        name: GTIdentifier::new((80, 85).into(), "Named".into()),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    doc: None,
                                                    name: "first".into(),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((101, 107).into()),
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    doc: None,
                                                    name: "last".into(),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((118, 124).into()),
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
                    exports: vec![
                        GTIdentifier::new((0, 5).into(), "Hello".into()),
                        GTIdentifier::new((62, 67).into(), "Hello".into()),
                        GTIdentifier::new((80, 85).into(), "Named".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_arrays() {
        let source_code = read_source_code("./examples/syntax/07-arrays.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![GTAlias {
                        doc: None,
                        name: GTIdentifier::new((0, 4).into(), "Book".into()),
                        descriptor: GTDescriptor::Object(GTObject {
                            extensions: vec![],
                            properties: vec![
                                GTProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: GTPrimitive::String((18, 24).into()).into(),
                                    required: true,
                                },
                                GTProperty {
                                    doc: None,
                                    name: "tags".into(),
                                    descriptor: GTDescriptor::Array(Box::new(GTArray {
                                        descriptor: GTPrimitive::String((34, 40).into()).into(),
                                    })),
                                    required: true,
                                },
                            ],
                        }),
                    }],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![GTIdentifier::new((0, 4).into(), "Book".into())],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_tuples() {
        let source_code = read_source_code("./examples/syntax/08-tuples.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 4).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTDescriptor::Tuple(GTTuple {
                                            descriptors: vec![
                                                GTPrimitive::String((18, 24).into()).into(),
                                                GTPrimitive::String((26, 32).into()).into(),
                                            ],
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "address".into(),
                                        descriptor: GTDescriptor::Tuple(GTTuple {
                                            descriptors: vec![
                                                GTPrimitive::Int((46, 49).into()).into(),
                                                GTPrimitive::String((51, 57).into()).into(),
                                                GTPrimitive::String((59, 65).into()).into(),
                                            ],
                                        }),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((70, 77).into(), "Address".into()),
                            descriptor: GTDescriptor::Tuple(GTTuple {
                                descriptors: vec![
                                    GTPrimitive::Int((81, 84).into()).into(),
                                    GTPrimitive::String((86, 92).into()).into(),
                                    GTPrimitive::String((94, 100).into()).into(),
                                ],
                            }),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 4).into(), "User".into()),
                        GTIdentifier::new((70, 77).into(), "Address".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_modules() {
        let source_code = read_source_code("./examples/syntax/09-modules.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![
                        GTImport {
                            path: "author".into(),
                            reference: GTImportReference::Glob,
                        },
                        GTImport {
                            path: "../../author".into(),
                            reference: GTImportReference::Names(vec![
                                GTImportName::Name(GTIdentifier::new(
                                    (31, 37).into(),
                                    "Author".into(),
                                )),
                                GTImportName::Name(GTIdentifier::new(
                                    (39, 44).into(),
                                    "Genre".into(),
                                )),
                                GTImportName::Alias(
                                    GTIdentifier::new((46, 55).into(), "Something".into()),
                                    GTIdentifier::new((59, 63).into(), "Else".into()),
                                ),
                            ]),
                        },
                        GTImport {
                            path: "author".into(),
                            reference: GTImportReference::Name(GTIdentifier::new(
                                (76, 82).into(),
                                "Author".into(),
                            )),
                        },
                    ],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((84, 88).into(), "Book".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "title".into(),
                                        descriptor: GTPrimitive::String((102, 108).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "author".into(),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            path: "../../author".into(),
                                            name: GTIdentifier::new((0, 0).into(), "Author".into()),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "genre".into(),
                                        descriptor: GTIdentifier::new(
                                            (148, 153).into(),
                                            "Genre".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((157, 163).into(), "Author".into()),
                            descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                path: "../../author".into(),
                                name: GTIdentifier::new((0, 0).into(), "Author".into()),
                            }),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::from_iter(vec!["author".into(), "../../author".into()]),
                    exports: vec![
                        GTIdentifier::new((84, 88).into(), "Book".into()),
                        GTIdentifier::new((157, 163).into(), "Author".into()),
                    ],
                    references: HashSet::from_iter(vec![GTIdentifier::new(
                        (148, 153).into(),
                        "Genre".into(),
                    )]),
                },
            },
        );
    }

    #[test]
    fn test_extensions() {
        let source_code = read_source_code("./examples/syntax/10-extensions.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 4).into(), "Base".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String((17, 23).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "age".into(),
                                        descriptor: GTPrimitive::Int((32, 35).into()).into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((39, 48).into(), "Processor".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![GTExtension {
                                    span: (55, 62).into(),
                                    reference: GTReference(GTIdentifier::new(
                                        (58, 62).into(),
                                        "Base".into(),
                                    )),
                                }],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "cores".into(),
                                    descriptor: GTPrimitive::Int((73, 76).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((80, 84).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![GTExtension {
                                    span: (91, 98).into(),
                                    reference: GTReference(GTIdentifier::new(
                                        (94, 98).into(),
                                        "Base".into(),
                                    )),
                                }],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "email".into(),
                                    descriptor: GTPrimitive::String((109, 115).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 4).into(), "Base".into()),
                        GTIdentifier::new((39, 48).into(), "Processor".into()),
                        GTIdentifier::new((80, 84).into(), "User".into()),
                    ],
                    references: HashSet::from_iter(vec![
                        GTIdentifier::new((58, 62).into(), "Base".into()),
                        GTIdentifier::new((94, 98).into(), "Base".into()),
                    ]),
                },
            },
        );
    }

    #[test]
    fn test_literals() {
        let source_code = read_source_code("./examples/syntax/11-literals.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 11).into(), "CommentBase".into()),
                            descriptor: GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "v".into(),
                                        descriptor: GTLiteral::Integer((21, 22).into(), 2).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "text".into(),
                                        descriptor: GTPrimitive::String((31, 37).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((41, 52).into(), "UserComment".into()),
                            descriptor: GTObject {
                                extensions: vec![GTExtension {
                                    span: (59, 73).into(),
                                    reference: GTReference(GTIdentifier::new(
                                        (62, 73).into(),
                                        "CommentBase".into(),
                                    )),
                                }],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "type".into(),
                                        descriptor: GTLiteral::String(
                                            (82, 88).into(),
                                            "user".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "userId".into(),
                                        descriptor: GTPrimitive::String((99, 105).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "published".into(),
                                        descriptor: GTPrimitive::Boolean((119, 126).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((130, 143).into(), "SystemComment".into()),
                            descriptor: GTObject {
                                extensions: vec![GTExtension {
                                    span: (150, 164).into(),
                                    reference: GTIdentifier::new(
                                        (153, 164).into(),
                                        "CommentBase".into(),
                                    )
                                    .into(),
                                }],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "type".into(),
                                        descriptor: GTLiteral::String(
                                            (173, 181).into(),
                                            "system".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "published".into(),
                                        descriptor: GTLiteral::Boolean((195, 199).into(), true)
                                            .into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((203, 208).into(), "False".into()),
                            descriptor: GTLiteral::Boolean((211, 216).into(), false).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((218, 223).into(), "Float".into()),
                            descriptor: GTLiteral::Float((226, 235).into(), 1.000_123).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((237, 243).into(), "Number".into()),
                            descriptor: GTLiteral::Integer((246, 255).into(), 1_234_567).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((257, 263).into(), "String".into()),
                            descriptor: GTLiteral::String(
                                (266, 288).into(),
                                "Hello, \\\"world\\\"! \\\\".into(),
                            )
                            .into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((290, 301).into(), "NegativeInt".into()),
                            descriptor: GTLiteral::Integer((304, 306).into(), -1).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((308, 321).into(), "NegativeFloat".into()),
                            descriptor: GTLiteral::Float((324, 328).into(), -1.0).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((330, 340).into(), "LargeFloat".into()),
                            descriptor: GTLiteral::Float((343, 346).into(), 1e6).into(),
                        },
                        GTAlias {
                            doc: None,
                            name: GTIdentifier::new((348, 358).into(), "SmallFloat".into()),
                            descriptor: GTLiteral::Float((361, 367).into(), 3.5e-4).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 11).into(), "CommentBase".into()),
                        GTIdentifier::new((41, 52).into(), "UserComment".into()),
                        GTIdentifier::new((130, 143).into(), "SystemComment".into()),
                        GTIdentifier::new((203, 208).into(), "False".into()),
                        GTIdentifier::new((218, 223).into(), "Float".into()),
                        GTIdentifier::new((237, 243).into(), "Number".into()),
                        GTIdentifier::new((257, 263).into(), "String".into()),
                        GTIdentifier::new((290, 301).into(), "NegativeInt".into()),
                        GTIdentifier::new((308, 321).into(), "NegativeFloat".into()),
                        GTIdentifier::new((330, 340).into(), "LargeFloat".into()),
                        GTIdentifier::new((348, 358).into(), "SmallFloat".into()),
                    ],
                    references: HashSet::from_iter(vec![
                        GTIdentifier::new((62, 73).into(), "CommentBase".into()),
                        GTIdentifier::new((153, 164).into(), "CommentBase".into()),
                    ]),
                },
            },
        );
    }

    fn read_source_code(path: &str) -> GTSourceCode {
        let content = fs::read_to_string(path).expect("cannot read file");
        GTSourceCode {
            name: path.into(),
            content,
        }
    }

    fn assert_module(source_code: GTSourceCode, expected: GTModuleParse) {
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(parse, expected);
    }
}
