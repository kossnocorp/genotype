use miette::Result;
use pest::iterators::Pair;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModuleParse {
    pub module: GTModule,
    pub resolve: GTResolve,
}

impl GTModule {
    pub fn parse<'a>(source_code: GTSourceCode) -> Result<GTModuleParse> {
        match parse_gt_code(&source_code.content) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => match Self::parse_pairs(pair) {
                    Ok(result) => Ok(GTModuleParse {
                        resolve: result.resolve,
                        module: GTModule {
                            source_code,
                            doc: result.doc,
                            imports: result.imports,
                            aliases: result.aliases,
                        },
                    }),

                    Err(error) => {
                        Err(GTModuleParseError::from_node_error(source_code, error).into())
                    }
                },

                None => {
                    let span = (0, source_code.content.len()).into();
                    Err(GTModuleParseError::from_node_error(
                        source_code,
                        GTNodeParseError::Internal(span, GTNode::Module),
                    )
                    .into())
                }
            },

            Err(error) => Err(GTModuleParseError::from_pest_error(source_code, error).into()),
        }
    }

    fn parse_pairs(module_pair: Pair<'_, Rule>) -> GTNodeParseResult<ModuleParseResult> {
        let mut doc: Option<GTDoc> = None;
        let mut imports = vec![];
        let mut aliases = vec![];
        let mut context = GTContext::new();

        for pair in module_pair.into_inner() {
            match pair.as_rule() {
                Rule::module_doc => {
                    let doc_pair = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                    if let Some(pair) = doc_pair {
                        doc = Some(if let Some(doc_pair) = doc {
                            doc_pair.concat(pair)
                        } else {
                            pair.into()
                        });
                    }
                }

                Rule::import => {
                    imports.push(GTImport::parse(pair, &mut context)?);
                }

                Rule::alias => {
                    aliases.push(GTAlias::parse(pair, &mut context)?);
                }

                Rule::EOI => {}

                _ => {
                    return Err(GTNodeParseError::Internal(
                        pair.as_span().into(),
                        GTNode::Module,
                    ))
                }
            }
        }

        Ok(ModuleParseResult {
            doc,
            imports,
            aliases,
            resolve: context.resolve,
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
                            span: (0, 9).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 3).into(), "Age".into()),
                            descriptor: GTPrimitive::Int((6, 9).into()).into(),
                        },
                        GTAlias {
                            span: (11, 27).into(),
                            doc: None,
                            attributes: vec![],
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
                            span: (0, 15).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 6).into(), "String".into()),
                            descriptor: GTPrimitive::String((9, 15).into()).into(),
                        },
                        GTAlias {
                            span: (17, 26).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((17, 20).into(), "Int".into()),
                            descriptor: GTPrimitive::Int((23, 26).into()).into(),
                        },
                        GTAlias {
                            span: (28, 41).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((28, 33).into(), "Float".into()),
                            descriptor: GTPrimitive::Float((36, 41).into()).into(),
                        },
                        GTAlias {
                            span: (43, 60).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((43, 50).into(), "Boolean".into()),
                            descriptor: GTPrimitive::Boolean((53, 60).into()).into(),
                        },
                        GTAlias {
                            span: (62, 73).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((62, 66).into(), "Null".into()),
                            descriptor: GTPrimitive::Null((69, 73).into()).into(),
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
                        GTIdentifier::new((62, 66).into(), "Null".into()),
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
                            span: (0, 26).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (8, 26).into(),
                                name: GTIdentifier::new((0, 5).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (12, 24).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((12, 16).into(), "name".into()),
                                    descriptor: GTPrimitive::String((18, 24).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            span: (28, 81).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((28, 33).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (36, 81).into(),
                                name: GTIdentifier::new((28, 33).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (40, 52).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((40, 44).into(), "name".into()),
                                        descriptor: GTPrimitive::String((46, 52).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (55, 63).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((55, 58).into(), "age".into()),
                                        descriptor: GTPrimitive::Int((60, 63).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (66, 79).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((66, 70).into(), "flag".into()),
                                        descriptor: GTDescriptor::Primitive(
                                            GTPrimitive::Boolean((72, 79).into()).into(),
                                        ),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            span: (83, 93).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((83, 88).into(), "Empty".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (91, 93).into(),
                                name: GTIdentifier::new((83, 88).into(), "Empty".into()).into(),
                                extensions: vec![],
                                properties: vec![],
                            }),
                        },
                        GTAlias {
                            span: (95, 109).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((95, 100).into(), "Empty".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (103, 109).into(),
                                name: GTIdentifier::new((95, 100).into(), "Empty".into()).into(),
                                extensions: vec![],
                                properties: vec![],
                            }),
                        },
                        GTAlias {
                            span: (111, 135).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((111, 116).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (119, 135).into(),
                                name: GTIdentifier::new((111, 116).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (121, 133).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((121, 125).into(), "name".into()),
                                    descriptor: GTPrimitive::String((127, 133).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            span: (137, 171).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((137, 142).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (145, 171).into(),
                                name: GTIdentifier::new((137, 142).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (147, 159).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((147, 151).into(), "name".into()),
                                        descriptor: GTPrimitive::String((153, 159).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (161, 169).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((161, 164).into(), "age".into()),
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
                    doc: Some(GTDoc::new(
                        (4, 38).into(),
                        "Module comment...\n...multiline".into(),
                    )),
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            span: (58, 111).into(),
                            doc: Some(GTDoc::new((62, 75).into(), "Alias comment".into())),
                            attributes: vec![],
                            name: GTIdentifier::new((76, 81).into(), "Hello".into()),
                            descriptor: GTPrimitive::String((105, 111).into()).into(),
                        },
                        GTAlias {
                            span: (113, 256).into(),
                            doc: Some(GTDoc::new(
                                (117, 150).into(),
                                "Multiline...\n...alias comment".into(),
                            )),
                            attributes: vec![],
                            name: GTIdentifier::new((151, 156).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (159, 256).into(),
                                name: GTIdentifier::new((151, 156).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (163, 198).into(),
                                        doc: Some(GTDoc::new(
                                            (167, 183).into(),
                                            "Property comment".into(),
                                        )),
                                        attributes: vec![],
                                        name: GTKey::new((186, 190).into(), "name".into()),
                                        descriptor: GTPrimitive::String((192, 198).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (201, 254).into(),
                                        doc: Some(GTDoc::new(
                                            (205, 243).into(),
                                            "Multiline...\n...property comment".into(),
                                        )),
                                        attributes: vec![],
                                        name: GTKey::new((246, 249).into(), "age".into()),
                                        descriptor: GTPrimitive::Int((251, 254).into()).into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            span: (258, 272).into(),
                            doc: None,
                            attributes: vec![],
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
                        span: (0, 38).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                        descriptor: GTDescriptor::Object(GTObject {
                            span: (8, 38).into(),
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()).into(),
                            extensions: vec![],
                            properties: vec![
                                GTProperty {
                                    span: (12, 24).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((12, 16).into(), "name".into()),
                                    descriptor: GTPrimitive::String((18, 24).into()).into(),

                                    required: true,
                                },
                                GTProperty {
                                    span: (27, 36).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((27, 30).into(), "age".into()),
                                    descriptor: GTPrimitive::Int((33, 36).into()).into(),
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
                            span: (0, 60).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (8, 60).into(),
                                name: GTIdentifier::new((0, 5).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (12, 58).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((12, 16).into(), "name".into()),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        span: (18, 58).into(),
                                        name: GTObjectName::Anonymous(
                                            (18, 58).into(),
                                            GTObjectNameParent::Property(
                                                GTIdentifier::new((0, 5).into(), "Hello".into()),
                                                vec![GTKey::new((12, 16).into(), "name".into())],
                                            ),
                                        ),
                                        extensions: vec![],
                                        properties: vec![
                                            GTProperty {
                                                span: (24, 37).into(),
                                                doc: None,
                                                attributes: vec![],
                                                name: GTKey::new((24, 29).into(), "first".into()),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String((31, 37).into()),
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                span: (42, 54).into(),
                                                doc: None,
                                                attributes: vec![],
                                                name: GTKey::new((42, 46).into(), "last".into()),
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
                            span: (62, 130).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((62, 67).into(), "Hello".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (70, 130).into(),
                                name: GTIdentifier::new((62, 67).into(), "Hello".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (74, 128).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((74, 78).into(), "name".into()),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        span: (80, 128).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTIdentifier::new((80, 85).into(), "Named".into()),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            span: (88, 128).into(),
                                            name: GTObjectName::Named(GTIdentifier::new(
                                                (80, 85).into(),
                                                "Named".into(),
                                            )),
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    span: (94, 107).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (94, 99).into(),
                                                        "first".into(),
                                                    ),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((101, 107).into()),
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (112, 124).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (112, 116).into(),
                                                        "last".into(),
                                                    ),
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
                        span: (0, 43).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTIdentifier::new((0, 4).into(), "Book".into()),
                        descriptor: GTDescriptor::Object(GTObject {
                            span: (7, 43).into(),
                            name: GTObjectName::Named(GTIdentifier::new(
                                (0, 4).into(),
                                "Book".into(),
                            )),
                            extensions: vec![],
                            properties: vec![
                                GTProperty {
                                    span: (11, 24).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((11, 16).into(), "title".into()),
                                    descriptor: GTPrimitive::String((18, 24).into()).into(),
                                    required: true,
                                },
                                GTProperty {
                                    span: (27, 41).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((27, 31).into(), "tags".into()),
                                    descriptor: GTDescriptor::Array(Box::new(GTArray {
                                        span: (33, 41).into(),
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
                            span: (0, 68).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 4).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (7, 68).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (0, 4).into(),
                                    "User".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (11, 33).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((11, 15).into(), "name".into()),
                                        descriptor: GTDescriptor::Tuple(GTTuple {
                                            span: (17, 33).into(),
                                            descriptors: vec![
                                                GTPrimitive::String((18, 24).into()).into(),
                                                GTPrimitive::String((26, 32).into()).into(),
                                            ],
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (36, 66).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((36, 43).into(), "address".into()),
                                        descriptor: GTDescriptor::Tuple(GTTuple {
                                            span: (45, 66).into(),
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
                            span: (70, 101).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((70, 77).into(), "Address".into()),
                            descriptor: GTDescriptor::Tuple(GTTuple {
                                span: (80, 101).into(),
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
                            span: (0, 12).into(),
                            path: GTPath::parse((4, 10).into(), "author").unwrap(),
                            reference: GTImportReference::Glob((11, 12).into()),
                        },
                        GTImport {
                            span: (13, 64).into(),
                            path: GTPath::parse((17, 29).into(), "../../author").unwrap(),
                            reference: GTImportReference::Names(
                                (30, 64).into(),
                                vec![
                                    GTImportName::Name(
                                        (31, 37).into(),
                                        GTIdentifier::new((31, 37).into(), "Author".into()),
                                    ),
                                    GTImportName::Name(
                                        (39, 44).into(),
                                        GTIdentifier::new((39, 44).into(), "Genre".into()),
                                    ),
                                    GTImportName::Alias(
                                        (46, 63).into(),
                                        GTIdentifier::new((46, 55).into(), "Something".into()),
                                        GTIdentifier::new((59, 63).into(), "Else".into()),
                                    ),
                                ],
                            ),
                        },
                        GTImport {
                            span: (65, 82).into(),
                            path: GTPath::parse((69, 75).into(), "author").unwrap(),
                            reference: GTImportReference::Name(
                                (76, 82).into(),
                                GTIdentifier::new((76, 82).into(), "Author".into()),
                            ),
                        },
                    ],
                    aliases: vec![
                        GTAlias {
                            span: (84, 155).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((84, 88).into(), "Book".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (91, 155).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (84, 88).into(),
                                    "Book".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (95, 108).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((95, 100).into(), "title".into()),
                                        descriptor: GTPrimitive::String((102, 108).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (111, 138).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((111, 117).into(), "author".into()),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            span: (119, 138).into(),
                                            path: GTPath::parse((119, 131).into(), "../../author")
                                                .unwrap(),
                                            name: GTIdentifier::new(
                                                (132, 138).into(),
                                                "Author".into(),
                                            ),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (141, 153).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((141, 146).into(), "genre".into()),
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
                            span: (157, 185).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((157, 163).into(), "Author".into()),
                            descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                span: (166, 185).into(),
                                path: GTPath::parse((166, 178).into(), "../../author").unwrap(),
                                name: GTIdentifier::new((179, 185).into(), "Author".into()),
                            }),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::from_iter(vec![
                        GTPath::parse((4, 10).into(), "author").unwrap(),
                        GTPath::parse((17, 29).into(), "../../author").unwrap(),
                        GTPath::parse((69, 75).into(), "author").unwrap(),
                        GTPath::parse((119, 131).into(), "../../author").unwrap(),
                        GTPath::parse((166, 178).into(), "../../author").unwrap(),
                    ]),
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
                            span: (0, 37).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 4).into(), "Base".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (7, 37).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (0, 4).into(),
                                    "Base".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (11, 23).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((11, 15).into(), "name".into()),
                                        descriptor: GTPrimitive::String((17, 23).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (27, 35).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((27, 30).into(), "age".into()),
                                        descriptor: GTPrimitive::Int((32, 35).into()).into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            span: (39, 78).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((39, 48).into(), "Processor".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (51, 78).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (39, 48).into(),
                                    "Processor".into(),
                                )),
                                extensions: vec![GTExtension {
                                    span: (55, 62).into(),
                                    reference: GTReference(
                                        (58, 62).into(),
                                        GTIdentifier::new((58, 62).into(), "Base".into()),
                                    ),
                                }],
                                properties: vec![GTProperty {
                                    span: (66, 76).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((66, 71).into(), "cores".into()),
                                    descriptor: GTPrimitive::Int((73, 76).into()).into(),
                                    required: true,
                                }],
                            }),
                        },
                        GTAlias {
                            span: (80, 117).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((80, 84).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (87, 117).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (80, 84).into(),
                                    "User".into(),
                                )),
                                extensions: vec![GTExtension {
                                    span: (91, 98).into(),
                                    reference: GTReference(
                                        (94, 98).into(),
                                        GTIdentifier::new((94, 98).into(), "Base".into()),
                                    ),
                                }],
                                properties: vec![GTProperty {
                                    span: (102, 115).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((102, 107).into(), "email".into()),
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
                            span: (0, 39).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 11).into(), "CommentBase".into()),
                            descriptor: GTObject {
                                span: (14, 39).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (0, 11).into(),
                                    "CommentBase".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (18, 22).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((18, 19).into(), "v".into()),
                                        descriptor: GTLiteral::Integer((21, 22).into(), 2).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (25, 37).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((25, 29).into(), "text".into()),
                                        descriptor: GTPrimitive::String((31, 37).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (41, 128).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((41, 52).into(), "UserComment".into()),
                            descriptor: GTObject {
                                span: (55, 128).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (41, 52).into(),
                                    "UserComment".into(),
                                )),
                                extensions: vec![GTExtension {
                                    span: (59, 73).into(),
                                    reference: GTReference(
                                        (62, 73).into(),
                                        GTIdentifier::new((62, 73).into(), "CommentBase".into()),
                                    ),
                                }],
                                properties: vec![
                                    GTProperty {
                                        span: (76, 88).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((76, 80).into(), "type".into()),
                                        descriptor: GTLiteral::String(
                                            (82, 88).into(),
                                            "user".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (91, 105).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((91, 97).into(), "userId".into()),
                                        descriptor: GTPrimitive::String((99, 105).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (108, 126).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((108, 117).into(), "published".into()),
                                        descriptor: GTPrimitive::Boolean((119, 126).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (130, 201).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((130, 143).into(), "SystemComment".into()),
                            descriptor: GTObject {
                                span: (146, 201).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (130, 143).into(),
                                    "SystemComment".into(),
                                )),
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
                                        span: (167, 181).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((167, 171).into(), "type".into()),
                                        descriptor: GTLiteral::String(
                                            (173, 181).into(),
                                            "system".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (184, 199).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((184, 193).into(), "published".into()),
                                        descriptor: GTLiteral::Boolean((195, 199).into(), true)
                                            .into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (203, 216).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((203, 208).into(), "False".into()),
                            descriptor: GTLiteral::Boolean((211, 216).into(), false).into(),
                        },
                        GTAlias {
                            span: (218, 235).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((218, 223).into(), "Float".into()),
                            descriptor: GTLiteral::Float((226, 235).into(), 1.000_123).into(),
                        },
                        GTAlias {
                            span: (237, 255).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((237, 243).into(), "Number".into()),
                            descriptor: GTLiteral::Integer((246, 255).into(), 1_234_567).into(),
                        },
                        GTAlias {
                            span: (257, 288).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((257, 263).into(), "String".into()),
                            descriptor: GTLiteral::String(
                                (266, 288).into(),
                                "Hello, \\\"world\\\"! \\\\".into(),
                            )
                            .into(),
                        },
                        GTAlias {
                            span: (290, 306).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((290, 301).into(), "NegativeInt".into()),
                            descriptor: GTLiteral::Integer((304, 306).into(), -1).into(),
                        },
                        GTAlias {
                            span: (308, 328).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((308, 321).into(), "NegativeFloat".into()),
                            descriptor: GTLiteral::Float((324, 328).into(), -1.0).into(),
                        },
                        GTAlias {
                            span: (330, 346).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((330, 340).into(), "LargeFloat".into()),
                            descriptor: GTLiteral::Float((343, 346).into(), 1e6).into(),
                        },
                        GTAlias {
                            span: (348, 367).into(),
                            doc: None,
                            attributes: vec![],
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

    #[test]
    fn test_unions() {
        let source_code = read_source_code("./examples/syntax/12-unions.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            span: (0, 25).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                            descriptor: GTUnion {
                                span: (8, 25).into(),
                                descriptors: vec![
                                    GTLiteral::String((8, 15).into(), "Sasha".into()).into(),
                                    GTLiteral::String((18, 25).into(), "world".into()).into(),
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((27, 36).into(), "Multiline".into()),
                            descriptor: GTUnion {
                                span: (41, 61).into(),
                                descriptors: vec![
                                    GTLiteral::String((43, 50).into(), "Hello".into()).into(),
                                    GTPrimitive::String((55, 61).into()).into(),
                                ],
                            }
                            .into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 5).into(), "Hello".into()),
                        GTIdentifier::new((27, 36).into(), "Multiline".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_attributes() {
        let source_code = read_source_code("./examples/syntax/13-attributes.type");
        assert_module(
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            span: (0, 25).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 5).into(), "Message".into()),
                            descriptor: GTUnion {
                                span: (8, 25).into(),
                                descriptors: vec![
                                    GTReference(
                                        (8, 15).into(),
                                        GTIdentifier::new((8, 15).into(), "Reply".into()),
                                    )
                                    .into(),
                                    GTReference(
                                        (8, 15).into(),
                                        GTIdentifier::new((8, 15).into(), "DM".into()),
                                    )
                                    .into(),
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((27, 36).into(), "Reply".into()),
                            descriptor: GTObject {
                                span: (41, 61).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (27, 36).into(),
                                    "Reply".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (45, 55).into(),
                                        doc: None,
                                        attributes: vec![GTAttribute {
                                            span: (0, 25).into(),
                                            name: GTAttributeName::new(
                                                (2, 12).into(),
                                                "tag".into(),
                                            ),
                                            descriptor: None,
                                        }],
                                        name: GTKey::new((45, 50).into(), "type".into()),
                                        descriptor: GTLiteral::String(
                                            (52, 55).into(),
                                            "reply".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (45, 55).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((45, 50).into(), "message".into()),
                                        descriptor: GTPrimitive::String((52, 55).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((27, 36).into(), "DM".into()),
                            descriptor: GTObject {
                                span: (41, 61).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (27, 36).into(),
                                    "Reply".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (45, 55).into(),
                                        doc: None,
                                        attributes: vec![GTAttribute {
                                            span: (0, 25).into(),
                                            name: GTAttributeName::new(
                                                (2, 12).into(),
                                                "tag".into(),
                                            ),
                                            descriptor: None,
                                        }],
                                        name: GTKey::new((45, 50).into(), "type".into()),
                                        descriptor: GTLiteral::String((52, 55).into(), "dm".into())
                                            .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (45, 55).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((45, 50).into(), "message".into()),
                                        descriptor: GTPrimitive::String((52, 55).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (0, 25).into(),
                                name: GTAttributeName::new((2, 12).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Assigment(
                                    GTAttributeAssignment::new(
                                        (0, 0).into(),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (0, 0).into(),
                                            "world".into(),
                                        )),
                                    ),
                                )),
                            }],
                            name: GTIdentifier::new((27, 36).into(), "Assignment".into()),
                            descriptor: GTLiteral::Integer((0, 0).into(), 123).into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (0, 25).into(),
                                name: GTAttributeName::new((2, 12).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Arguments(vec![
                                    GTAttributeValue::Literal(GTLiteral::String(
                                        (0, 0).into(),
                                        "cruel".into(),
                                    )),
                                    GTAttributeValue::Literal(GTLiteral::String(
                                        (0, 0).into(),
                                        "world".into(),
                                    )),
                                ])),
                            }],
                            name: GTIdentifier::new((27, 36).into(), "Arguments".into()),
                            descriptor: GTLiteral::Boolean((0, 0).into(), true).into(),
                        },
                        GTAlias {
                            span: (27, 61).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (0, 25).into(),
                                name: GTAttributeName::new((2, 12).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Properties(vec![
                                    GTAttributeProperty::new(
                                        (0, 0).into(),
                                        GTAttributeKey::new((0, 5).into(), "which".into()),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (0, 0).into(),
                                            "cruel".into(),
                                        )),
                                    ),
                                    GTAttributeProperty::new(
                                        (0, 0).into(),
                                        GTAttributeKey::new((0, 5).into(), "what".into()),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (0, 0).into(),
                                            "world".into(),
                                        )),
                                    ),
                                ])),
                            }],
                            name: GTIdentifier::new((27, 36).into(), "Properties".into()),
                            descriptor: GTLiteral::Boolean((0, 0).into(), true).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 5).into(), "Message".into()),
                        GTIdentifier::new((27, 36).into(), "Reply".into()),
                        GTIdentifier::new((27, 36).into(), "DM".into()),
                        GTIdentifier::new((27, 36).into(), "Assingmnet".into()),
                        GTIdentifier::new((27, 36).into(), "Arguments".into()),
                        GTIdentifier::new((27, 36).into(), "Properties".into()),
                    ],
                    references: HashSet::new(),
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
