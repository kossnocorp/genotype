use miette::{NamedSource, Result};
use pest::iterators::Pair;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModuleParse {
    pub module: GTModule,
    pub resolve: GTResolve,
}

impl GTModule {
    pub fn parse<'a>(id: String, source_code: NamedSource<String>) -> Result<GTModuleParse> {
        let id = GTModuleId(id);
        match parse_gt_code(source_code.inner()) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => match Self::parse_pairs(id.clone(), pair) {
                    Ok(result) => Ok(GTModuleParse {
                        resolve: result.resolve,
                        module: GTModule {
                            id,
                            source_code,
                            doc: result.doc,
                            imports: result.imports,
                            aliases: result.aliases,
                        },
                    }),

                    Err(error) => Err(error.with_source_code(source_code)),
                },

                None => {
                    let span = (0, source_code.inner().len()).into();
                    Err(GTModuleParseError::from_node_error(
                        source_code,
                        GTParseError::Internal(span, GTNode::Module),
                    )
                    .into())
                }
            },

            Err(error) => Err(GTModuleParseError::from_pest_error(source_code, error).into()),
        }
    }

    fn parse_pairs(
        module_id: GTModuleId,
        module_pair: Pair<'_, Rule>,
    ) -> Result<ModuleParseResult> {
        let mut doc: Option<GTDoc> = None;
        let mut imports = vec![];
        let mut aliases = vec![];
        let mut context = GTContext::new(module_id);

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
                    return Err(
                        GTParseError::Internal(pair.as_span().into(), GTNode::Module).into(),
                    )
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
    use miette::NamedSource;
    use pretty_assertions::assert_eq;
    use std::{collections::HashSet, fs};

    use super::*;

    #[test]
    fn test_alias() {
        let source_code = read_source_code("../examples/01-syntax/01-alias.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Age".into()),
                            span: (0, 9).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 3).into(), "Age".into()),
                            descriptor: GTPrimitive::Int((6, 9).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "AnotherAge".into()),
                            span: (11, 27).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((11, 21).into(), "AnotherAge".into()),
                            descriptor: GTReference {
                                span: (24, 27).into(),
                                id: GTReferenceId("module".into(), (24, 27).into()),
                                definition_id: GTReferenceDefinitionId::Unresolved,
                                identifier: GTIdentifier::new((24, 27).into(), "Age".into()),
                            }
                            .into(),
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
        let source_code = read_source_code("../examples/01-syntax/02-primitives.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "String".into()),
                            span: (0, 15).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 6).into(), "String".into()),
                            descriptor: GTPrimitive::String((9, 15).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Int".into()),
                            span: (17, 26).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((17, 20).into(), "Int".into()),
                            descriptor: GTPrimitive::Int((23, 26).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Float".into()),
                            span: (28, 41).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((28, 33).into(), "Float".into()),
                            descriptor: GTPrimitive::Float((36, 41).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Boolean".into()),
                            span: (43, 60).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((43, 50).into(), "Boolean".into()),
                            descriptor: GTPrimitive::Boolean((53, 60).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Null".into()),
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
        let source_code = read_source_code("../examples/01-syntax/03-objects.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                            id: GTDefinitionId("module".into(), "Empty".into()),
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
                            id: GTDefinitionId("module".into(), "Empty".into()),
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
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
        let source_code = read_source_code("../examples/01-syntax/04-comments.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: Some(GTDoc::new(
                        (4, 38).into(),
                        "Module comment...\n...multiline".into(),
                    )),
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Hello".into()),
                            span: (58, 111).into(),
                            doc: Some(GTDoc::new((62, 75).into(), "Alias comment".into())),
                            attributes: vec![],
                            name: GTIdentifier::new((76, 81).into(), "Hello".into()),
                            descriptor: GTPrimitive::String((105, 111).into()).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
        let source_code = read_source_code("../examples/01-syntax/05-optional.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![GTAlias {
                        id: GTDefinitionId("module".into(), "Hello".into()),
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
        let source_code = read_source_code("../examples/01-syntax/06-nested.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                                        name: GTObjectName::Alias(
                                            GTIdentifier::new((18, 58).into(), "HelloName".into()),
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
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                                        id: GTDefinitionId("module".into(), "Named".into()),
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
        let source_code = read_source_code("../examples/01-syntax/07-arrays.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![GTAlias {
                        id: GTDefinitionId("module".into(), "Book".into()),
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
        let source_code = read_source_code("../examples/01-syntax/08-tuples.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "User".into()),
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
                            id: GTDefinitionId("module".into(), "Address".into()),
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
        let source_code = read_source_code("../examples/01-syntax/09-modules.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
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
                            id: GTDefinitionId("module".into(), "Book".into()),
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
                                        descriptor: GTReference {
                                            span: (148, 153).into(),
                                            id: GTReferenceId("module".into(), (148, 153).into()),
                                            definition_id: GTReferenceDefinitionId::Unresolved,
                                            identifier: GTIdentifier::new(
                                                (148, 153).into(),
                                                "Genre".into(),
                                            ),
                                        }
                                        .into(),
                                        required: true,
                                    },
                                ],
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Author".into()),
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
        let source_code = read_source_code("../examples/01-syntax/10-extensions.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Base".into()),
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
                            id: GTDefinitionId("module".into(), "Processor".into()),
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
                                    reference: GTReference {
                                        span: (58, 62).into(),
                                        id: GTReferenceId("module".into(), (58, 62).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new(
                                            (58, 62).into(),
                                            "Base".into(),
                                        ),
                                    },
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
                            id: GTDefinitionId("module".into(), "User".into()),
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
                                    reference: GTReference {
                                        span: (94, 98).into(),
                                        id: GTReferenceId("module".into(), (94, 98).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new(
                                            (94, 98).into(),
                                            "Base".into(),
                                        ),
                                    },
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
        let source_code = read_source_code("../examples/01-syntax/11-literals.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "CommentBase".into()),
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
                            id: GTDefinitionId("module".into(), "UserComment".into()),
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
                                    reference: GTReference {
                                        span: (62, 73).into(),
                                        id: GTReferenceId("module".into(), (62, 73).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new(
                                            (62, 73).into(),
                                            "CommentBase".into(),
                                        ),
                                    },
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
                            id: GTDefinitionId("module".into(), "SystemComment".into()),
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
                                    reference: GTReference {
                                        span: (153, 164).into(),
                                        id: GTReferenceId("module".into(), (153, 164).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new(
                                            (153, 164).into(),
                                            "CommentBase".into(),
                                        ),
                                    },
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
                            id: GTDefinitionId("module".into(), "False".into()),
                            span: (203, 216).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((203, 208).into(), "False".into()),
                            descriptor: GTLiteral::Boolean((211, 216).into(), false).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Float".into()),
                            span: (218, 235).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((218, 223).into(), "Float".into()),
                            descriptor: GTLiteral::Float((226, 235).into(), 1.000_123).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Number".into()),
                            span: (237, 255).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((237, 243).into(), "Number".into()),
                            descriptor: GTLiteral::Integer((246, 255).into(), 1_234_567).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "String".into()),
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
                            id: GTDefinitionId("module".into(), "NegativeInt".into()),
                            span: (290, 306).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((290, 301).into(), "NegativeInt".into()),
                            descriptor: GTLiteral::Integer((304, 306).into(), -1).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "NegativeFloat".into()),
                            span: (308, 328).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((308, 321).into(), "NegativeFloat".into()),
                            descriptor: GTLiteral::Float((324, 328).into(), -1.0).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "LargeFloat".into()),
                            span: (330, 346).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((330, 340).into(), "LargeFloat".into()),
                            descriptor: GTLiteral::Float((343, 346).into(), 1e6).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "SmallFloat".into()),
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
        let source_code = read_source_code("../examples/01-syntax/12-unions.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Hello".into()),
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
                            id: GTDefinitionId("module".into(), "Multiline".into()),
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
        let source_code = read_source_code("../examples/01-syntax/13-attributes.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Message".into()),
                            span: (0, 20).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 7).into(), "Message".into()),
                            descriptor: GTUnion {
                                span: (10, 20).into(),
                                descriptors: vec![
                                    GTReference {
                                        span: (10, 15).into(),
                                        id: GTReferenceId("module".into(), (10, 15).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new(
                                            (10, 15).into(),
                                            "Reply".into(),
                                        ),
                                    }
                                    .into(),
                                    GTReference {
                                        span: (18, 20).into(),
                                        id: GTReferenceId("module".into(), (18, 20).into()),
                                        definition_id: GTReferenceDefinitionId::Unresolved,
                                        identifier: GTIdentifier::new((18, 20).into(), "DM".into()),
                                    }
                                    .into(),
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Reply".into()),
                            span: (22, 77).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((22, 27).into(), "Reply".into()),
                            descriptor: GTObject {
                                span: (30, 77).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (22, 27).into(),
                                    "Reply".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (34, 56).into(),
                                        doc: None,
                                        attributes: vec![GTAttribute {
                                            span: (34, 40).into(),
                                            name: GTAttributeName::new(
                                                (36, 39).into(),
                                                "tag".into(),
                                            ),
                                            descriptor: None,
                                        }],
                                        name: GTKey::new((43, 47).into(), "type".into()),
                                        descriptor: GTLiteral::String(
                                            (49, 56).into(),
                                            "reply".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (60, 75).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((60, 67).into(), "message".into()),
                                        descriptor: GTPrimitive::String((69, 75).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "DM".into()),
                            span: (79, 128).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((79, 81).into(), "DM".into()),
                            descriptor: GTObject {
                                span: (84, 128).into(),
                                name: GTObjectName::Named(GTIdentifier::new(
                                    (79, 81).into(),
                                    "DM".into(),
                                )),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (88, 107).into(),
                                        doc: None,
                                        attributes: vec![GTAttribute {
                                            span: (88, 94).into(),
                                            name: GTAttributeName::new(
                                                (90, 93).into(),
                                                "tag".into(),
                                            ),
                                            descriptor: None,
                                        }],
                                        name: GTKey::new((97, 101).into(), "type".into()),
                                        descriptor: GTLiteral::String(
                                            (103, 107).into(),
                                            "dm".into(),
                                        )
                                        .into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (111, 126).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((111, 118).into(), "message".into()),
                                        descriptor: GTPrimitive::String((120, 126).into()).into(),
                                        required: true,
                                    },
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Assignment".into()),
                            span: (130, 165).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (130, 148).into(),
                                name: GTAttributeName::new((132, 137).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Assignment(
                                    GTAttributeAssignment::new(
                                        (138, 147).into(),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (140, 147).into(),
                                            "world".into(),
                                        )),
                                    ),
                                )),
                            }],
                            name: GTIdentifier::new((149, 159).into(), "Assignment".into()),
                            descriptor: GTLiteral::Integer((162, 165).into(), 123).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Arguments".into()),
                            span: (167, 210).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (167, 193).into(),
                                name: GTAttributeName::new((169, 174).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Arguments(vec![
                                    GTAttributeValue::Literal(GTLiteral::String(
                                        (175, 182).into(),
                                        "cruel".into(),
                                    )),
                                    GTAttributeValue::Literal(GTLiteral::String(
                                        (184, 191).into(),
                                        "world".into(),
                                    )),
                                ])),
                            }],
                            name: GTIdentifier::new((194, 203).into(), "Arguments".into()),
                            descriptor: GTLiteral::Boolean((206, 210).into(), true).into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Properties".into()),
                            span: (212, 271).into(),
                            doc: None,
                            attributes: vec![GTAttribute {
                                span: (212, 253).into(),
                                name: GTAttributeName::new((214, 219).into(), "hello".into()),
                                descriptor: Some(GTAttributeDescriptor::Properties(vec![
                                    GTAttributeProperty::new(
                                        (220, 235).into(),
                                        GTAttributeKey::new((220, 225).into(), "which".into()),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (228, 235).into(),
                                            "cruel".into(),
                                        )),
                                    ),
                                    GTAttributeProperty::new(
                                        (237, 251).into(),
                                        GTAttributeKey::new((237, 241).into(), "what".into()),
                                        GTAttributeValue::Literal(GTLiteral::String(
                                            (244, 251).into(),
                                            "world".into(),
                                        )),
                                    ),
                                ])),
                            }],
                            name: GTIdentifier::new((254, 264).into(), "Properties".into()),
                            descriptor: GTLiteral::Boolean((267, 271).into(), true).into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 7).into(), "Message".into()),
                        GTIdentifier::new((22, 27).into(), "Reply".into()),
                        GTIdentifier::new((79, 81).into(), "DM".into()),
                        GTIdentifier::new((149, 159).into(), "Assignment".into()),
                        GTIdentifier::new((194, 203).into(), "Arguments".into()),
                        GTIdentifier::new((254, 264).into(), "Properties".into()),
                    ],
                    references: HashSet::from_iter(vec![
                        GTIdentifier::new((10, 15).into(), "Reply".into()),
                        GTIdentifier::new((18, 20).into(), "DM".into()),
                    ]),
                },
            },
        );
    }

    #[test]
    fn test_records() {
        let source_code = read_source_code("../examples/01-syntax/14-records.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Dict".into()),
                            span: (0, 21).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 4).into(), "Dict".into()),
                            descriptor: GTRecord {
                                span: (7, 21).into(),
                                key: GTRecordKey::String((9, 11).into()),
                                descriptor: GTPrimitive::String((13, 19).into()).into(),
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Map".into()),
                            span: (23, 46).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((23, 26).into(), "Map".into()),
                            descriptor: GTRecord {
                                span: (29, 46).into(),
                                key: GTRecordKey::Int((31, 36).into()),
                                descriptor: GTPrimitive::String((38, 44).into()).into(),
                            }
                            .into(),
                        },
                    ],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![
                        GTIdentifier::new((0, 4).into(), "Dict".into()),
                        GTIdentifier::new((23, 26).into(), "Map".into()),
                    ],
                    references: HashSet::new(),
                },
            },
        );
    }

    #[test]
    fn test_any() {
        let source_code = read_source_code("../examples/01-syntax/15-any.type");
        assert_module(
            "module".into(),
            source_code.clone(),
            GTModuleParse {
                module: GTModule {
                    id: "module".into(),
                    source_code,
                    doc: None,
                    imports: vec![],
                    aliases: vec![GTAlias {
                        id: GTDefinitionId("module".into(), "Anything".into()),
                        span: (0, 14).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTIdentifier::new((0, 8).into(), "Anything".into()),
                        descriptor: GTAny((11, 14).into()).into(),
                    }],
                },
                resolve: GTResolve {
                    deps: HashSet::new(),
                    exports: vec![GTIdentifier::new((0, 8).into(), "Anything".into())],
                    references: HashSet::new(),
                },
            },
        );
    }

    fn read_source_code(path: &str) -> NamedSource<String> {
        let content = fs::read_to_string(path).expect("cannot read file");
        NamedSource::new(path, content)
    }

    fn assert_module(id: String, source_code: NamedSource<String>, expected: GTModuleParse) {
        let parse = GTModule::parse(id, source_code).unwrap();
        assert_eq!(parse, expected);
    }
}
