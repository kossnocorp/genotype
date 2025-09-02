use miette::{NamedSource, Result};
use pest::iterators::Pair;

use crate::*;

/// Module parse result. It contains the module tree and resolve data.
#[derive(Debug, PartialEq, Clone)]
pub struct GTModuleParse {
    /// Module tree.
    pub module: GTModule,
    /// Module resolve. It contains module meta information used to build
    /// the dependency graph.
    pub resolve: GTModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GTModule {
    pub fn parse<'a>(id: GTModuleId, source_code: NamedSource<String>) -> Result<GTModuleParse> {
        match parse_gt_code(source_code.inner()) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => match Self::parse_pairs(id.clone(), pair) {
                    Ok(result) => Ok(GTModuleParse {
                        resolve: result.resolve,
                        module: GTModule {
                            id,
                            doc: result.doc,
                            imports: result.imports,
                            aliases: result.aliases,
                        },
                        source_code,
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
                    );
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
    resolve: GTModuleResolve,
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use miette::NamedSource;
    use std::fs;

    use super::*;

    #[test]
    fn test_alias() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/01-alias.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Age",
                        ),
                        span: GTSpan(
                            0,
                            9,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                3,
                            ),
                            "Age",
                        ),
                        descriptor: Primitive(
                            Int64(
                                GTSpan(
                                    6,
                                    9,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "AnotherAge",
                        ),
                        span: GTSpan(
                            11,
                            27,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                11,
                                21,
                            ),
                            "AnotherAge",
                        ),
                        descriptor: Reference(
                            GTReference {
                                span: GTSpan(
                                    24,
                                    27,
                                ),
                                id: GTReferenceId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    GTSpan(
                                        24,
                                        27,
                                    ),
                                ),
                                definition_id: Unresolved,
                                identifier: GTIdentifier(
                                    GTSpan(
                                        24,
                                        27,
                                    ),
                                    "Age",
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "snake_case",
                        ),
                        span: GTSpan(
                            29,
                            45,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                29,
                                39,
                            ),
                            "snake_case",
                        ),
                        descriptor: Primitive(
                            Int64(
                                GTSpan(
                                    42,
                                    45,
                                ),
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            3,
                        ),
                        "Age",
                    ),
                    GTIdentifier(
                        GTSpan(
                            11,
                            21,
                        ),
                        "AnotherAge",
                    ),
                    GTIdentifier(
                        GTSpan(
                            29,
                            39,
                        ),
                        "snake_case",
                    ),
                ],
                references: {
                    GTIdentifier(
                        GTSpan(
                            24,
                            27,
                        ),
                        "Age",
                    ),
                },
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/01-alias.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_primitives() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/02-primitives.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "String",
                        ),
                        span: GTSpan(
                            0,
                            15,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                6,
                            ),
                            "String",
                        ),
                        descriptor: Primitive(
                            String(
                                GTSpan(
                                    9,
                                    15,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int",
                        ),
                        span: GTSpan(
                            17,
                            26,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                17,
                                20,
                            ),
                            "Int",
                        ),
                        descriptor: Primitive(
                            Int64(
                                GTSpan(
                                    23,
                                    26,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float",
                        ),
                        span: GTSpan(
                            28,
                            41,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                28,
                                33,
                            ),
                            "Float",
                        ),
                        descriptor: Primitive(
                            Float64(
                                GTSpan(
                                    36,
                                    41,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Boolean",
                        ),
                        span: GTSpan(
                            43,
                            60,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                43,
                                50,
                            ),
                            "Boolean",
                        ),
                        descriptor: Primitive(
                            Boolean(
                                GTSpan(
                                    53,
                                    60,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Null",
                        ),
                        span: GTSpan(
                            62,
                            73,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                62,
                                66,
                            ),
                            "Null",
                        ),
                        descriptor: Primitive(
                            Null(
                                GTSpan(
                                    69,
                                    73,
                                ),
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            6,
                        ),
                        "String",
                    ),
                    GTIdentifier(
                        GTSpan(
                            17,
                            20,
                        ),
                        "Int",
                    ),
                    GTIdentifier(
                        GTSpan(
                            28,
                            33,
                        ),
                        "Float",
                    ),
                    GTIdentifier(
                        GTSpan(
                            43,
                            50,
                        ),
                        "Boolean",
                    ),
                    GTIdentifier(
                        GTSpan(
                            62,
                            66,
                        ),
                        "Null",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/02-primitives.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_objects() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/03-objects.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            0,
                            26,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    8,
                                    26,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            5,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            12,
                                            24,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                12,
                                                16,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    18,
                                                    24,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            28,
                            81,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                28,
                                33,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    36,
                                    81,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            28,
                                            33,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            40,
                                            52,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                40,
                                                44,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    46,
                                                    52,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            55,
                                            63,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                55,
                                                58,
                                            ),
                                            "age",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    60,
                                                    63,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            66,
                                            79,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                66,
                                                70,
                                            ),
                                            "flag",
                                        ),
                                        descriptor: Primitive(
                                            Boolean(
                                                GTSpan(
                                                    72,
                                                    79,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Empty",
                        ),
                        span: GTSpan(
                            83,
                            93,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                83,
                                88,
                            ),
                            "Empty",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    91,
                                    93,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            83,
                                            88,
                                        ),
                                        "Empty",
                                    ),
                                ),
                                extensions: [],
                                properties: [],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Empty",
                        ),
                        span: GTSpan(
                            95,
                            107,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                95,
                                100,
                            ),
                            "Empty",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    103,
                                    107,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            95,
                                            100,
                                        ),
                                        "Empty",
                                    ),
                                ),
                                extensions: [],
                                properties: [],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            109,
                            133,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                109,
                                114,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    117,
                                    133,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            109,
                                            114,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            119,
                                            131,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                119,
                                                123,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    125,
                                                    131,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            135,
                            169,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                135,
                                140,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    143,
                                    169,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            135,
                                            140,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            145,
                                            157,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                145,
                                                149,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    151,
                                                    157,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            159,
                                            167,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                159,
                                                162,
                                            ),
                                            "age",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    164,
                                                    167,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "PascalCase",
                        ),
                        span: GTSpan(
                            171,
                            205,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                171,
                                181,
                            ),
                            "PascalCase",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    184,
                                    205,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            171,
                                            181,
                                        ),
                                        "PascalCase",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            188,
                                            203,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                188,
                                                198,
                                            ),
                                            "snake_case",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    200,
                                                    203,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            28,
                            33,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            83,
                            88,
                        ),
                        "Empty",
                    ),
                    GTIdentifier(
                        GTSpan(
                            95,
                            100,
                        ),
                        "Empty",
                    ),
                    GTIdentifier(
                        GTSpan(
                            109,
                            114,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            135,
                            140,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            171,
                            181,
                        ),
                        "PascalCase",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/03-objects.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_comments() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/04-comments.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: Some(
                    GTDoc(
                        GTSpan(
                            4,
                            38,
                        ),
                        "Module comment...\n...multiline",
                    ),
                ),
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            58,
                            111,
                        ),
                        doc: Some(
                            GTDoc(
                                GTSpan(
                                    62,
                                    75,
                                ),
                                "Alias comment",
                            ),
                        ),
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                76,
                                81,
                            ),
                            "Hello",
                        ),
                        descriptor: Primitive(
                            String(
                                GTSpan(
                                    105,
                                    111,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            113,
                            256,
                        ),
                        doc: Some(
                            GTDoc(
                                GTSpan(
                                    117,
                                    150,
                                ),
                                "Multiline...\n...alias comment",
                            ),
                        ),
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                151,
                                156,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    159,
                                    256,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            151,
                                            156,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            163,
                                            198,
                                        ),
                                        doc: Some(
                                            GTDoc(
                                                GTSpan(
                                                    167,
                                                    183,
                                                ),
                                                "Property comment",
                                            ),
                                        ),
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                186,
                                                190,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    192,
                                                    198,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            201,
                                            254,
                                        ),
                                        doc: Some(
                                            GTDoc(
                                                GTSpan(
                                                    205,
                                                    243,
                                                ),
                                                "Multiline...\n...property comment",
                                            ),
                                        ),
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                246,
                                                249,
                                            ),
                                            "age",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    251,
                                                    254,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            258,
                            272,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                258,
                                263,
                            ),
                            "Hello",
                        ),
                        descriptor: Primitive(
                            String(
                                GTSpan(
                                    266,
                                    272,
                                ),
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            76,
                            81,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            151,
                            156,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            258,
                            263,
                        ),
                        "Hello",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/04-comments.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_optional() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/05-optional.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            0,
                            38,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    8,
                                    38,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            5,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            12,
                                            24,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                12,
                                                16,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    18,
                                                    24,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            27,
                                            36,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                27,
                                                30,
                                            ),
                                            "age",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    33,
                                                    36,
                                                ),
                                            ),
                                        ),
                                        required: false,
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "Hello",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/05-optional.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_nested() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/06-nested.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            0,
                            60,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    8,
                                    60,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            5,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            12,
                                            58,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                12,
                                                16,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Object(
                                            GTObject {
                                                span: GTSpan(
                                                    18,
                                                    58,
                                                ),
                                                name: Alias(
                                                    GTIdentifier(
                                                        GTSpan(
                                                            18,
                                                            58,
                                                        ),
                                                        "HelloName",
                                                    ),
                                                    Property(
                                                        GTIdentifier(
                                                            GTSpan(
                                                                0,
                                                                5,
                                                            ),
                                                            "Hello",
                                                        ),
                                                        [
                                                            GTKey(
                                                                GTSpan(
                                                                    12,
                                                                    16,
                                                                ),
                                                                "name",
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                                extensions: [],
                                                properties: [
                                                    GTProperty {
                                                        span: GTSpan(
                                                            24,
                                                            37,
                                                        ),
                                                        doc: None,
                                                        attributes: [],
                                                        name: GTKey(
                                                            GTSpan(
                                                                24,
                                                                29,
                                                            ),
                                                            "first",
                                                        ),
                                                        descriptor: Primitive(
                                                            String(
                                                                GTSpan(
                                                                    31,
                                                                    37,
                                                                ),
                                                            ),
                                                        ),
                                                        required: true,
                                                    },
                                                    GTProperty {
                                                        span: GTSpan(
                                                            42,
                                                            54,
                                                        ),
                                                        doc: None,
                                                        attributes: [],
                                                        name: GTKey(
                                                            GTSpan(
                                                                42,
                                                                46,
                                                            ),
                                                            "last",
                                                        ),
                                                        descriptor: Primitive(
                                                            String(
                                                                GTSpan(
                                                                    48,
                                                                    54,
                                                                ),
                                                            ),
                                                        ),
                                                        required: true,
                                                    },
                                                ],
                                            },
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            62,
                            130,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                62,
                                67,
                            ),
                            "Hello",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    70,
                                    130,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            62,
                                            67,
                                        ),
                                        "Hello",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            74,
                                            128,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                74,
                                                78,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Alias(
                                            GTAlias {
                                                id: GTDefinitionId(
                                                    GTModuleId(
                                                        "module",
                                                    ),
                                                    "Named",
                                                ),
                                                span: GTSpan(
                                                    80,
                                                    128,
                                                ),
                                                doc: None,
                                                attributes: [],
                                                name: GTIdentifier(
                                                    GTSpan(
                                                        80,
                                                        85,
                                                    ),
                                                    "Named",
                                                ),
                                                descriptor: Object(
                                                    GTObject {
                                                        span: GTSpan(
                                                            88,
                                                            128,
                                                        ),
                                                        name: Named(
                                                            GTIdentifier(
                                                                GTSpan(
                                                                    80,
                                                                    85,
                                                                ),
                                                                "Named",
                                                            ),
                                                        ),
                                                        extensions: [],
                                                        properties: [
                                                            GTProperty {
                                                                span: GTSpan(
                                                                    94,
                                                                    107,
                                                                ),
                                                                doc: None,
                                                                attributes: [],
                                                                name: GTKey(
                                                                    GTSpan(
                                                                        94,
                                                                        99,
                                                                    ),
                                                                    "first",
                                                                ),
                                                                descriptor: Primitive(
                                                                    String(
                                                                        GTSpan(
                                                                            101,
                                                                            107,
                                                                        ),
                                                                    ),
                                                                ),
                                                                required: true,
                                                            },
                                                            GTProperty {
                                                                span: GTSpan(
                                                                    112,
                                                                    124,
                                                                ),
                                                                doc: None,
                                                                attributes: [],
                                                                name: GTKey(
                                                                    GTSpan(
                                                                        112,
                                                                        116,
                                                                    ),
                                                                    "last",
                                                                ),
                                                                descriptor: Primitive(
                                                                    String(
                                                                        GTSpan(
                                                                            118,
                                                                            124,
                                                                        ),
                                                                    ),
                                                                ),
                                                                required: true,
                                                            },
                                                        ],
                                                    },
                                                ),
                                            },
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            62,
                            67,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            80,
                            85,
                        ),
                        "Named",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/06-nested.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_arrays() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/07-arrays.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Book",
                        ),
                        span: GTSpan(
                            0,
                            43,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                4,
                            ),
                            "Book",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    7,
                                    43,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            4,
                                        ),
                                        "Book",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            11,
                                            24,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                11,
                                                16,
                                            ),
                                            "title",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    18,
                                                    24,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            27,
                                            41,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                27,
                                                31,
                                            ),
                                            "tags",
                                        ),
                                        descriptor: Array(
                                            GTArray {
                                                span: GTSpan(
                                                    33,
                                                    41,
                                                ),
                                                descriptor: Primitive(
                                                    String(
                                                        GTSpan(
                                                            34,
                                                            40,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            4,
                        ),
                        "Book",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/07-arrays.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_tuples() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/08-tuples.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "User",
                        ),
                        span: GTSpan(
                            0,
                            68,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                4,
                            ),
                            "User",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    7,
                                    68,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            4,
                                        ),
                                        "User",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            11,
                                            33,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                11,
                                                15,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Tuple(
                                            GTTuple {
                                                span: GTSpan(
                                                    17,
                                                    33,
                                                ),
                                                descriptors: [
                                                    Primitive(
                                                        String(
                                                            GTSpan(
                                                                18,
                                                                24,
                                                            ),
                                                        ),
                                                    ),
                                                    Primitive(
                                                        String(
                                                            GTSpan(
                                                                26,
                                                                32,
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            36,
                                            66,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                36,
                                                43,
                                            ),
                                            "address",
                                        ),
                                        descriptor: Tuple(
                                            GTTuple {
                                                span: GTSpan(
                                                    45,
                                                    66,
                                                ),
                                                descriptors: [
                                                    Primitive(
                                                        Int64(
                                                            GTSpan(
                                                                46,
                                                                49,
                                                            ),
                                                        ),
                                                    ),
                                                    Primitive(
                                                        String(
                                                            GTSpan(
                                                                51,
                                                                57,
                                                            ),
                                                        ),
                                                    ),
                                                    Primitive(
                                                        String(
                                                            GTSpan(
                                                                59,
                                                                65,
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Address",
                        ),
                        span: GTSpan(
                            70,
                            101,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                70,
                                77,
                            ),
                            "Address",
                        ),
                        descriptor: Tuple(
                            GTTuple {
                                span: GTSpan(
                                    80,
                                    101,
                                ),
                                descriptors: [
                                    Primitive(
                                        Int64(
                                            GTSpan(
                                                81,
                                                84,
                                            ),
                                        ),
                                    ),
                                    Primitive(
                                        String(
                                            GTSpan(
                                                86,
                                                92,
                                            ),
                                        ),
                                    ),
                                    Primitive(
                                        String(
                                            GTSpan(
                                                94,
                                                100,
                                            ),
                                        ),
                                    ),
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            4,
                        ),
                        "User",
                    ),
                    GTIdentifier(
                        GTSpan(
                            70,
                            77,
                        ),
                        "Address",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/08-tuples.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_modules() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/09-modules.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [
                    GTImport {
                        span: GTSpan(
                            0,
                            12,
                        ),
                        path: GTPath(
                            GTSpan(
                                4,
                                10,
                            ),
                            Unresolved,
                            "author",
                        ),
                        reference: Glob(
                            GTSpan(
                                11,
                                12,
                            ),
                        ),
                    },
                    GTImport {
                        span: GTSpan(
                            13,
                            64,
                        ),
                        path: GTPath(
                            GTSpan(
                                17,
                                29,
                            ),
                            Unresolved,
                            "../../author",
                        ),
                        reference: Names(
                            GTSpan(
                                30,
                                64,
                            ),
                            [
                                Name(
                                    GTSpan(
                                        31,
                                        37,
                                    ),
                                    GTIdentifier(
                                        GTSpan(
                                            31,
                                            37,
                                        ),
                                        "Author",
                                    ),
                                ),
                                Name(
                                    GTSpan(
                                        39,
                                        44,
                                    ),
                                    GTIdentifier(
                                        GTSpan(
                                            39,
                                            44,
                                        ),
                                        "Genre",
                                    ),
                                ),
                                Alias(
                                    GTSpan(
                                        46,
                                        63,
                                    ),
                                    GTIdentifier(
                                        GTSpan(
                                            46,
                                            55,
                                        ),
                                        "Something",
                                    ),
                                    GTIdentifier(
                                        GTSpan(
                                            59,
                                            63,
                                        ),
                                        "Else",
                                    ),
                                ),
                            ],
                        ),
                    },
                    GTImport {
                        span: GTSpan(
                            65,
                            82,
                        ),
                        path: GTPath(
                            GTSpan(
                                69,
                                75,
                            ),
                            Unresolved,
                            "author",
                        ),
                        reference: Name(
                            GTSpan(
                                76,
                                82,
                            ),
                            GTIdentifier(
                                GTSpan(
                                    76,
                                    82,
                                ),
                                "Author",
                            ),
                        ),
                    },
                ],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Book",
                        ),
                        span: GTSpan(
                            84,
                            155,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                84,
                                88,
                            ),
                            "Book",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    91,
                                    155,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            84,
                                            88,
                                        ),
                                        "Book",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            95,
                                            108,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                95,
                                                100,
                                            ),
                                            "title",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    102,
                                                    108,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            111,
                                            138,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                111,
                                                117,
                                            ),
                                            "author",
                                        ),
                                        descriptor: InlineImport(
                                            GTInlineImport {
                                                span: GTSpan(
                                                    119,
                                                    138,
                                                ),
                                                name: GTIdentifier(
                                                    GTSpan(
                                                        132,
                                                        138,
                                                    ),
                                                    "Author",
                                                ),
                                                path: GTPath(
                                                    GTSpan(
                                                        119,
                                                        131,
                                                    ),
                                                    Unresolved,
                                                    "../../author",
                                                ),
                                            },
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            141,
                                            153,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                141,
                                                146,
                                            ),
                                            "genre",
                                        ),
                                        descriptor: Reference(
                                            GTReference {
                                                span: GTSpan(
                                                    148,
                                                    153,
                                                ),
                                                id: GTReferenceId(
                                                    GTModuleId(
                                                        "module",
                                                    ),
                                                    GTSpan(
                                                        148,
                                                        153,
                                                    ),
                                                ),
                                                definition_id: Unresolved,
                                                identifier: GTIdentifier(
                                                    GTSpan(
                                                        148,
                                                        153,
                                                    ),
                                                    "Genre",
                                                ),
                                            },
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Author",
                        ),
                        span: GTSpan(
                            157,
                            185,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                157,
                                163,
                            ),
                            "Author",
                        ),
                        descriptor: InlineImport(
                            GTInlineImport {
                                span: GTSpan(
                                    166,
                                    185,
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        179,
                                        185,
                                    ),
                                    "Author",
                                ),
                                path: GTPath(
                                    GTSpan(
                                        166,
                                        178,
                                    ),
                                    Unresolved,
                                    "../../author",
                                ),
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {
                    GTPath(
                        GTSpan(
                            166,
                            178,
                        ),
                        Unresolved,
                        "../../author",
                    ),
                    GTPath(
                        GTSpan(
                            17,
                            29,
                        ),
                        Unresolved,
                        "../../author",
                    ),
                    GTPath(
                        GTSpan(
                            4,
                            10,
                        ),
                        Unresolved,
                        "author",
                    ),
                    GTPath(
                        GTSpan(
                            69,
                            75,
                        ),
                        Unresolved,
                        "author",
                    ),
                    GTPath(
                        GTSpan(
                            119,
                            131,
                        ),
                        Unresolved,
                        "../../author",
                    ),
                },
                exports: [
                    GTIdentifier(
                        GTSpan(
                            84,
                            88,
                        ),
                        "Book",
                    ),
                    GTIdentifier(
                        GTSpan(
                            157,
                            163,
                        ),
                        "Author",
                    ),
                ],
                references: {
                    GTIdentifier(
                        GTSpan(
                            148,
                            153,
                        ),
                        "Genre",
                    ),
                },
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/09-modules.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_extensions() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/10-extensions.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Base",
                        ),
                        span: GTSpan(
                            0,
                            37,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                4,
                            ),
                            "Base",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    7,
                                    37,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            4,
                                        ),
                                        "Base",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            11,
                                            23,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                11,
                                                15,
                                            ),
                                            "name",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    17,
                                                    23,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            27,
                                            35,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                27,
                                                30,
                                            ),
                                            "age",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    32,
                                                    35,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Processor",
                        ),
                        span: GTSpan(
                            39,
                            78,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                39,
                                48,
                            ),
                            "Processor",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    51,
                                    78,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            39,
                                            48,
                                        ),
                                        "Processor",
                                    ),
                                ),
                                extensions: [
                                    GTExtension {
                                        span: GTSpan(
                                            55,
                                            62,
                                        ),
                                        reference: GTReference {
                                            span: GTSpan(
                                                58,
                                                62,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    58,
                                                    62,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    58,
                                                    62,
                                                ),
                                                "Base",
                                            ),
                                        },
                                    },
                                ],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            66,
                                            76,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                66,
                                                71,
                                            ),
                                            "cores",
                                        ),
                                        descriptor: Primitive(
                                            Int64(
                                                GTSpan(
                                                    73,
                                                    76,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "User",
                        ),
                        span: GTSpan(
                            80,
                            117,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                80,
                                84,
                            ),
                            "User",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    87,
                                    117,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            80,
                                            84,
                                        ),
                                        "User",
                                    ),
                                ),
                                extensions: [
                                    GTExtension {
                                        span: GTSpan(
                                            91,
                                            98,
                                        ),
                                        reference: GTReference {
                                            span: GTSpan(
                                                94,
                                                98,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    94,
                                                    98,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    94,
                                                    98,
                                                ),
                                                "Base",
                                            ),
                                        },
                                    },
                                ],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            102,
                                            115,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                102,
                                                107,
                                            ),
                                            "email",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    109,
                                                    115,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            4,
                        ),
                        "Base",
                    ),
                    GTIdentifier(
                        GTSpan(
                            39,
                            48,
                        ),
                        "Processor",
                    ),
                    GTIdentifier(
                        GTSpan(
                            80,
                            84,
                        ),
                        "User",
                    ),
                ],
                references: {
                    GTIdentifier(
                        GTSpan(
                            94,
                            98,
                        ),
                        "Base",
                    ),
                    GTIdentifier(
                        GTSpan(
                            58,
                            62,
                        ),
                        "Base",
                    ),
                },
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/10-extensions.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_literals() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/11-literals.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "CommentBase",
                        ),
                        span: GTSpan(
                            0,
                            39,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                11,
                            ),
                            "CommentBase",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    14,
                                    39,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            0,
                                            11,
                                        ),
                                        "CommentBase",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            18,
                                            22,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                18,
                                                19,
                                            ),
                                            "v",
                                        ),
                                        descriptor: Literal(
                                            Integer(
                                                GTSpan(
                                                    21,
                                                    22,
                                                ),
                                                2,
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            25,
                                            37,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                25,
                                                29,
                                            ),
                                            "text",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    31,
                                                    37,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "UserComment",
                        ),
                        span: GTSpan(
                            41,
                            128,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                41,
                                52,
                            ),
                            "UserComment",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    55,
                                    128,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            41,
                                            52,
                                        ),
                                        "UserComment",
                                    ),
                                ),
                                extensions: [
                                    GTExtension {
                                        span: GTSpan(
                                            59,
                                            73,
                                        ),
                                        reference: GTReference {
                                            span: GTSpan(
                                                62,
                                                73,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    62,
                                                    73,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    62,
                                                    73,
                                                ),
                                                "CommentBase",
                                            ),
                                        },
                                    },
                                ],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            76,
                                            88,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                76,
                                                80,
                                            ),
                                            "type",
                                        ),
                                        descriptor: Literal(
                                            String(
                                                GTSpan(
                                                    82,
                                                    88,
                                                ),
                                                "user",
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            91,
                                            105,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                91,
                                                97,
                                            ),
                                            "userId",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    99,
                                                    105,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            108,
                                            126,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                108,
                                                117,
                                            ),
                                            "published",
                                        ),
                                        descriptor: Primitive(
                                            Boolean(
                                                GTSpan(
                                                    119,
                                                    126,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "SystemComment",
                        ),
                        span: GTSpan(
                            130,
                            201,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                130,
                                143,
                            ),
                            "SystemComment",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    146,
                                    201,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            130,
                                            143,
                                        ),
                                        "SystemComment",
                                    ),
                                ),
                                extensions: [
                                    GTExtension {
                                        span: GTSpan(
                                            150,
                                            164,
                                        ),
                                        reference: GTReference {
                                            span: GTSpan(
                                                153,
                                                164,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    153,
                                                    164,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    153,
                                                    164,
                                                ),
                                                "CommentBase",
                                            ),
                                        },
                                    },
                                ],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            167,
                                            181,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                167,
                                                171,
                                            ),
                                            "type",
                                        ),
                                        descriptor: Literal(
                                            String(
                                                GTSpan(
                                                    173,
                                                    181,
                                                ),
                                                "system",
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            184,
                                            199,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                184,
                                                193,
                                            ),
                                            "published",
                                        ),
                                        descriptor: Literal(
                                            Boolean(
                                                GTSpan(
                                                    195,
                                                    199,
                                                ),
                                                true,
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "False",
                        ),
                        span: GTSpan(
                            203,
                            216,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                203,
                                208,
                            ),
                            "False",
                        ),
                        descriptor: Literal(
                            Boolean(
                                GTSpan(
                                    211,
                                    216,
                                ),
                                false,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float",
                        ),
                        span: GTSpan(
                            218,
                            235,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                218,
                                223,
                            ),
                            "Float",
                        ),
                        descriptor: Literal(
                            Float(
                                GTSpan(
                                    226,
                                    235,
                                ),
                                1.000123,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Number",
                        ),
                        span: GTSpan(
                            237,
                            255,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                237,
                                243,
                            ),
                            "Number",
                        ),
                        descriptor: Literal(
                            Integer(
                                GTSpan(
                                    246,
                                    255,
                                ),
                                1234567,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "String",
                        ),
                        span: GTSpan(
                            257,
                            288,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                257,
                                263,
                            ),
                            "String",
                        ),
                        descriptor: Literal(
                            String(
                                GTSpan(
                                    266,
                                    288,
                                ),
                                "Hello, \\\"world\\\"! \\\\",
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "NegativeInt",
                        ),
                        span: GTSpan(
                            290,
                            306,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                290,
                                301,
                            ),
                            "NegativeInt",
                        ),
                        descriptor: Literal(
                            Integer(
                                GTSpan(
                                    304,
                                    306,
                                ),
                                -1,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "NegativeFloat",
                        ),
                        span: GTSpan(
                            308,
                            328,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                308,
                                321,
                            ),
                            "NegativeFloat",
                        ),
                        descriptor: Literal(
                            Float(
                                GTSpan(
                                    324,
                                    328,
                                ),
                                -1.0,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "LargeFloat",
                        ),
                        span: GTSpan(
                            330,
                            346,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                330,
                                340,
                            ),
                            "LargeFloat",
                        ),
                        descriptor: Literal(
                            Float(
                                GTSpan(
                                    343,
                                    346,
                                ),
                                1000000.0,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "SmallFloat",
                        ),
                        span: GTSpan(
                            348,
                            367,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                348,
                                358,
                            ),
                            "SmallFloat",
                        ),
                        descriptor: Literal(
                            Float(
                                GTSpan(
                                    361,
                                    367,
                                ),
                                0.00035,
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            11,
                        ),
                        "CommentBase",
                    ),
                    GTIdentifier(
                        GTSpan(
                            41,
                            52,
                        ),
                        "UserComment",
                    ),
                    GTIdentifier(
                        GTSpan(
                            130,
                            143,
                        ),
                        "SystemComment",
                    ),
                    GTIdentifier(
                        GTSpan(
                            203,
                            208,
                        ),
                        "False",
                    ),
                    GTIdentifier(
                        GTSpan(
                            218,
                            223,
                        ),
                        "Float",
                    ),
                    GTIdentifier(
                        GTSpan(
                            237,
                            243,
                        ),
                        "Number",
                    ),
                    GTIdentifier(
                        GTSpan(
                            257,
                            263,
                        ),
                        "String",
                    ),
                    GTIdentifier(
                        GTSpan(
                            290,
                            301,
                        ),
                        "NegativeInt",
                    ),
                    GTIdentifier(
                        GTSpan(
                            308,
                            321,
                        ),
                        "NegativeFloat",
                    ),
                    GTIdentifier(
                        GTSpan(
                            330,
                            340,
                        ),
                        "LargeFloat",
                    ),
                    GTIdentifier(
                        GTSpan(
                            348,
                            358,
                        ),
                        "SmallFloat",
                    ),
                ],
                references: {
                    GTIdentifier(
                        GTSpan(
                            62,
                            73,
                        ),
                        "CommentBase",
                    ),
                    GTIdentifier(
                        GTSpan(
                            153,
                            164,
                        ),
                        "CommentBase",
                    ),
                },
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/11-literals.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_unions() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/12-unions.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            0,
                            25,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "Hello",
                        ),
                        descriptor: Union(
                            GTUnion {
                                span: GTSpan(
                                    8,
                                    25,
                                ),
                                descriptors: [
                                    Literal(
                                        String(
                                            GTSpan(
                                                8,
                                                15,
                                            ),
                                            "Sasha",
                                        ),
                                    ),
                                    Literal(
                                        String(
                                            GTSpan(
                                                18,
                                                25,
                                            ),
                                            "world",
                                        ),
                                    ),
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Multiline",
                        ),
                        span: GTSpan(
                            27,
                            61,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                27,
                                36,
                            ),
                            "Multiline",
                        ),
                        descriptor: Union(
                            GTUnion {
                                span: GTSpan(
                                    41,
                                    61,
                                ),
                                descriptors: [
                                    Literal(
                                        String(
                                            GTSpan(
                                                43,
                                                50,
                                            ),
                                            "Hello",
                                        ),
                                    ),
                                    Primitive(
                                        String(
                                            GTSpan(
                                                55,
                                                61,
                                            ),
                                        ),
                                    ),
                                ],
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            27,
                            36,
                        ),
                        "Multiline",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/12-unions.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_attributes() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/13-attributes.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Message",
                        ),
                        span: GTSpan(
                            0,
                            20,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                7,
                            ),
                            "Message",
                        ),
                        descriptor: Union(
                            GTUnion {
                                span: GTSpan(
                                    10,
                                    20,
                                ),
                                descriptors: [
                                    Reference(
                                        GTReference {
                                            span: GTSpan(
                                                10,
                                                15,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    10,
                                                    15,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    10,
                                                    15,
                                                ),
                                                "Reply",
                                            ),
                                        },
                                    ),
                                    Reference(
                                        GTReference {
                                            span: GTSpan(
                                                18,
                                                20,
                                            ),
                                            id: GTReferenceId(
                                                GTModuleId(
                                                    "module",
                                                ),
                                                GTSpan(
                                                    18,
                                                    20,
                                                ),
                                            ),
                                            definition_id: Unresolved,
                                            identifier: GTIdentifier(
                                                GTSpan(
                                                    18,
                                                    20,
                                                ),
                                                "DM",
                                            ),
                                        },
                                    ),
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Reply",
                        ),
                        span: GTSpan(
                            22,
                            77,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                22,
                                27,
                            ),
                            "Reply",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    30,
                                    77,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            22,
                                            27,
                                        ),
                                        "Reply",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            34,
                                            56,
                                        ),
                                        doc: None,
                                        attributes: [
                                            GTAttribute {
                                                span: GTSpan(
                                                    34,
                                                    40,
                                                ),
                                                name: GTAttributeName {
                                                    span: GTSpan(
                                                        36,
                                                        39,
                                                    ),
                                                    name: "tag",
                                                },
                                                descriptor: None,
                                            },
                                        ],
                                        name: GTKey(
                                            GTSpan(
                                                43,
                                                47,
                                            ),
                                            "type",
                                        ),
                                        descriptor: Literal(
                                            String(
                                                GTSpan(
                                                    49,
                                                    56,
                                                ),
                                                "reply",
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            60,
                                            75,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                60,
                                                67,
                                            ),
                                            "message",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    69,
                                                    75,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "DM",
                        ),
                        span: GTSpan(
                            79,
                            128,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                79,
                                81,
                            ),
                            "DM",
                        ),
                        descriptor: Object(
                            GTObject {
                                span: GTSpan(
                                    84,
                                    128,
                                ),
                                name: Named(
                                    GTIdentifier(
                                        GTSpan(
                                            79,
                                            81,
                                        ),
                                        "DM",
                                    ),
                                ),
                                extensions: [],
                                properties: [
                                    GTProperty {
                                        span: GTSpan(
                                            88,
                                            107,
                                        ),
                                        doc: None,
                                        attributes: [
                                            GTAttribute {
                                                span: GTSpan(
                                                    88,
                                                    94,
                                                ),
                                                name: GTAttributeName {
                                                    span: GTSpan(
                                                        90,
                                                        93,
                                                    ),
                                                    name: "tag",
                                                },
                                                descriptor: None,
                                            },
                                        ],
                                        name: GTKey(
                                            GTSpan(
                                                97,
                                                101,
                                            ),
                                            "type",
                                        ),
                                        descriptor: Literal(
                                            String(
                                                GTSpan(
                                                    103,
                                                    107,
                                                ),
                                                "dm",
                                            ),
                                        ),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: GTSpan(
                                            111,
                                            126,
                                        ),
                                        doc: None,
                                        attributes: [],
                                        name: GTKey(
                                            GTSpan(
                                                111,
                                                118,
                                            ),
                                            "message",
                                        ),
                                        descriptor: Primitive(
                                            String(
                                                GTSpan(
                                                    120,
                                                    126,
                                                ),
                                            ),
                                        ),
                                        required: true,
                                    },
                                ],
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Assignment",
                        ),
                        span: GTSpan(
                            130,
                            165,
                        ),
                        doc: None,
                        attributes: [
                            GTAttribute {
                                span: GTSpan(
                                    130,
                                    148,
                                ),
                                name: GTAttributeName {
                                    span: GTSpan(
                                        132,
                                        137,
                                    ),
                                    name: "hello",
                                },
                                descriptor: Some(
                                    Assignment(
                                        GTAttributeAssignment {
                                            span: GTSpan(
                                                138,
                                                147,
                                            ),
                                            value: Literal(
                                                String(
                                                    GTSpan(
                                                        140,
                                                        147,
                                                    ),
                                                    "world",
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ],
                        name: GTIdentifier(
                            GTSpan(
                                149,
                                159,
                            ),
                            "Assignment",
                        ),
                        descriptor: Literal(
                            Integer(
                                GTSpan(
                                    162,
                                    165,
                                ),
                                123,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Arguments",
                        ),
                        span: GTSpan(
                            167,
                            210,
                        ),
                        doc: None,
                        attributes: [
                            GTAttribute {
                                span: GTSpan(
                                    167,
                                    193,
                                ),
                                name: GTAttributeName {
                                    span: GTSpan(
                                        169,
                                        174,
                                    ),
                                    name: "hello",
                                },
                                descriptor: Some(
                                    Arguments(
                                        [
                                            Literal(
                                                String(
                                                    GTSpan(
                                                        175,
                                                        182,
                                                    ),
                                                    "cruel",
                                                ),
                                            ),
                                            Literal(
                                                String(
                                                    GTSpan(
                                                        184,
                                                        191,
                                                    ),
                                                    "world",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        ],
                        name: GTIdentifier(
                            GTSpan(
                                194,
                                203,
                            ),
                            "Arguments",
                        ),
                        descriptor: Literal(
                            Boolean(
                                GTSpan(
                                    206,
                                    210,
                                ),
                                true,
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Properties",
                        ),
                        span: GTSpan(
                            212,
                            271,
                        ),
                        doc: None,
                        attributes: [
                            GTAttribute {
                                span: GTSpan(
                                    212,
                                    253,
                                ),
                                name: GTAttributeName {
                                    span: GTSpan(
                                        214,
                                        219,
                                    ),
                                    name: "hello",
                                },
                                descriptor: Some(
                                    Properties(
                                        [
                                            GTAttributeProperty {
                                                span: GTSpan(
                                                    220,
                                                    235,
                                                ),
                                                name: GTAttributeKey {
                                                    span: GTSpan(
                                                        220,
                                                        225,
                                                    ),
                                                    name: "which",
                                                },
                                                value: Literal(
                                                    String(
                                                        GTSpan(
                                                            228,
                                                            235,
                                                        ),
                                                        "cruel",
                                                    ),
                                                ),
                                            },
                                            GTAttributeProperty {
                                                span: GTSpan(
                                                    237,
                                                    251,
                                                ),
                                                name: GTAttributeKey {
                                                    span: GTSpan(
                                                        237,
                                                        241,
                                                    ),
                                                    name: "what",
                                                },
                                                value: Literal(
                                                    String(
                                                        GTSpan(
                                                            244,
                                                            251,
                                                        ),
                                                        "world",
                                                    ),
                                                ),
                                            },
                                        ],
                                    ),
                                ),
                            },
                        ],
                        name: GTIdentifier(
                            GTSpan(
                                254,
                                264,
                            ),
                            "Properties",
                        ),
                        descriptor: Literal(
                            Boolean(
                                GTSpan(
                                    267,
                                    271,
                                ),
                                true,
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            7,
                        ),
                        "Message",
                    ),
                    GTIdentifier(
                        GTSpan(
                            22,
                            27,
                        ),
                        "Reply",
                    ),
                    GTIdentifier(
                        GTSpan(
                            79,
                            81,
                        ),
                        "DM",
                    ),
                    GTIdentifier(
                        GTSpan(
                            149,
                            159,
                        ),
                        "Assignment",
                    ),
                    GTIdentifier(
                        GTSpan(
                            194,
                            203,
                        ),
                        "Arguments",
                    ),
                    GTIdentifier(
                        GTSpan(
                            254,
                            264,
                        ),
                        "Properties",
                    ),
                ],
                references: {
                    GTIdentifier(
                        GTSpan(
                            10,
                            15,
                        ),
                        "Reply",
                    ),
                    GTIdentifier(
                        GTSpan(
                            18,
                            20,
                        ),
                        "DM",
                    ),
                },
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/13-attributes.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_records() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/14-records.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Dict",
                        ),
                        span: GTSpan(
                            0,
                            21,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                4,
                            ),
                            "Dict",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    7,
                                    21,
                                ),
                                key: String(
                                    GTSpan(
                                        9,
                                        11,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            13,
                                            19,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Map",
                        ),
                        span: GTSpan(
                            23,
                            46,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                23,
                                26,
                            ),
                            "Map",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    29,
                                    46,
                                ),
                                key: Int64(
                                    GTSpan(
                                        31,
                                        36,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            38,
                                            44,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            4,
                        ),
                        "Dict",
                    ),
                    GTIdentifier(
                        GTSpan(
                            23,
                            26,
                        ),
                        "Map",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/14-records.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_any() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/15-any.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Anything",
                        ),
                        span: GTSpan(
                            0,
                            14,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                8,
                            ),
                            "Anything",
                        ),
                        descriptor: Any(
                            GTAny(
                                GTSpan(
                                    11,
                                    14,
                                ),
                            ),
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            8,
                        ),
                        "Anything",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/15-any.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_branded() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/16-branded.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "OrgId",
                        ),
                        span: GTSpan(
                            0,
                            12,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "OrgId",
                        ),
                        descriptor: Branded(
                            GTBranded {
                                span: GTSpan(
                                    8,
                                    12,
                                ),
                                id: GTDefinitionId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    "OrgId",
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        0,
                                        5,
                                    ),
                                    "OrgId",
                                ),
                                primitive: Int64(
                                    GTSpan(
                                        9,
                                        12,
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "UserId",
                        ),
                        span: GTSpan(
                            14,
                            30,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                14,
                                20,
                            ),
                            "UserId",
                        ),
                        descriptor: Branded(
                            GTBranded {
                                span: GTSpan(
                                    23,
                                    30,
                                ),
                                id: GTDefinitionId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    "UserId",
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        14,
                                        20,
                                    ),
                                    "UserId",
                                ),
                                primitive: String(
                                    GTSpan(
                                        24,
                                        30,
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Const",
                        ),
                        span: GTSpan(
                            32,
                            46,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                32,
                                37,
                            ),
                            "Const",
                        ),
                        descriptor: Branded(
                            GTBranded {
                                span: GTSpan(
                                    40,
                                    46,
                                ),
                                id: GTDefinitionId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    "Const",
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        32,
                                        37,
                                    ),
                                    "Const",
                                ),
                                primitive: Float64(
                                    GTSpan(
                                        41,
                                        46,
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Yes",
                        ),
                        span: GTSpan(
                            48,
                            62,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                48,
                                51,
                            ),
                            "Yes",
                        ),
                        descriptor: Branded(
                            GTBranded {
                                span: GTSpan(
                                    54,
                                    62,
                                ),
                                id: GTDefinitionId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    "Yes",
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        48,
                                        51,
                                    ),
                                    "Yes",
                                ),
                                primitive: Boolean(
                                    GTSpan(
                                        55,
                                        62,
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Nope",
                        ),
                        span: GTSpan(
                            64,
                            76,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                64,
                                68,
                            ),
                            "Nope",
                        ),
                        descriptor: Branded(
                            GTBranded {
                                span: GTSpan(
                                    71,
                                    76,
                                ),
                                id: GTDefinitionId(
                                    GTModuleId(
                                        "module",
                                    ),
                                    "Nope",
                                ),
                                name: GTIdentifier(
                                    GTSpan(
                                        64,
                                        68,
                                    ),
                                    "Nope",
                                ),
                                primitive: Null(
                                    GTSpan(
                                        72,
                                        76,
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "OrgId",
                    ),
                    GTIdentifier(
                        GTSpan(
                            14,
                            20,
                        ),
                        "UserId",
                    ),
                    GTIdentifier(
                        GTSpan(
                            32,
                            37,
                        ),
                        "Const",
                    ),
                    GTIdentifier(
                        GTSpan(
                            48,
                            51,
                        ),
                        "Yes",
                    ),
                    GTIdentifier(
                        GTSpan(
                            64,
                            68,
                        ),
                        "Nope",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/16-branded.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_number_sizes() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/17-number_sizes.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int8",
                        ),
                        span: GTSpan(
                            0,
                            9,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                4,
                            ),
                            "Int8",
                        ),
                        descriptor: Primitive(
                            Int8(
                                GTSpan(
                                    7,
                                    9,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int16",
                        ),
                        span: GTSpan(
                            10,
                            21,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                10,
                                15,
                            ),
                            "Int16",
                        ),
                        descriptor: Primitive(
                            Int16(
                                GTSpan(
                                    18,
                                    21,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int32",
                        ),
                        span: GTSpan(
                            22,
                            33,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                22,
                                27,
                            ),
                            "Int32",
                        ),
                        descriptor: Primitive(
                            Int32(
                                GTSpan(
                                    30,
                                    33,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int64",
                        ),
                        span: GTSpan(
                            34,
                            45,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                34,
                                39,
                            ),
                            "Int64",
                        ),
                        descriptor: Primitive(
                            Int64(
                                GTSpan(
                                    42,
                                    45,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int128",
                        ),
                        span: GTSpan(
                            46,
                            59,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                46,
                                52,
                            ),
                            "Int128",
                        ),
                        descriptor: Primitive(
                            Int128(
                                GTSpan(
                                    55,
                                    59,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntSize",
                        ),
                        span: GTSpan(
                            60,
                            75,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                60,
                                67,
                            ),
                            "IntSize",
                        ),
                        descriptor: Primitive(
                            IntSize(
                                GTSpan(
                                    70,
                                    75,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU8",
                        ),
                        span: GTSpan(
                            76,
                            86,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                76,
                                81,
                            ),
                            "IntU8",
                        ),
                        descriptor: Primitive(
                            IntU8(
                                GTSpan(
                                    84,
                                    86,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU16",
                        ),
                        span: GTSpan(
                            87,
                            99,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                87,
                                93,
                            ),
                            "IntU16",
                        ),
                        descriptor: Primitive(
                            IntU16(
                                GTSpan(
                                    96,
                                    99,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU32",
                        ),
                        span: GTSpan(
                            100,
                            112,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                100,
                                106,
                            ),
                            "IntU32",
                        ),
                        descriptor: Primitive(
                            IntU32(
                                GTSpan(
                                    109,
                                    112,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU64",
                        ),
                        span: GTSpan(
                            113,
                            125,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                113,
                                119,
                            ),
                            "IntU64",
                        ),
                        descriptor: Primitive(
                            IntU64(
                                GTSpan(
                                    122,
                                    125,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU128",
                        ),
                        span: GTSpan(
                            126,
                            140,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                126,
                                133,
                            ),
                            "IntU128",
                        ),
                        descriptor: Primitive(
                            IntU128(
                                GTSpan(
                                    136,
                                    140,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntUSize",
                        ),
                        span: GTSpan(
                            141,
                            157,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                141,
                                149,
                            ),
                            "IntUSize",
                        ),
                        descriptor: Primitive(
                            IntUSize(
                                GTSpan(
                                    152,
                                    157,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float32",
                        ),
                        span: GTSpan(
                            158,
                            171,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                158,
                                165,
                            ),
                            "Float32",
                        ),
                        descriptor: Primitive(
                            Float32(
                                GTSpan(
                                    168,
                                    171,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float64",
                        ),
                        span: GTSpan(
                            172,
                            185,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                172,
                                179,
                            ),
                            "Float64",
                        ),
                        descriptor: Primitive(
                            Float64(
                                GTSpan(
                                    182,
                                    185,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int8Record",
                        ),
                        span: GTSpan(
                            187,
                            216,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                187,
                                197,
                            ),
                            "Int8Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    200,
                                    216,
                                ),
                                key: Int8(
                                    GTSpan(
                                        202,
                                        206,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            208,
                                            214,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int16Record",
                        ),
                        span: GTSpan(
                            217,
                            248,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                217,
                                228,
                            ),
                            "Int16Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    231,
                                    248,
                                ),
                                key: Int16(
                                    GTSpan(
                                        233,
                                        238,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            240,
                                            246,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int32Record",
                        ),
                        span: GTSpan(
                            249,
                            280,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                249,
                                260,
                            ),
                            "Int32Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    263,
                                    280,
                                ),
                                key: Int32(
                                    GTSpan(
                                        265,
                                        270,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            272,
                                            278,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int64Record",
                        ),
                        span: GTSpan(
                            281,
                            312,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                281,
                                292,
                            ),
                            "Int64Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    295,
                                    312,
                                ),
                                key: Int64(
                                    GTSpan(
                                        297,
                                        302,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            304,
                                            310,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Int128Record",
                        ),
                        span: GTSpan(
                            313,
                            346,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                313,
                                325,
                            ),
                            "Int128Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    328,
                                    346,
                                ),
                                key: Int128(
                                    GTSpan(
                                        330,
                                        336,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            338,
                                            344,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntSizeRecord",
                        ),
                        span: GTSpan(
                            347,
                            382,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                347,
                                360,
                            ),
                            "IntSizeRecord",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    363,
                                    382,
                                ),
                                key: IntSize(
                                    GTSpan(
                                        365,
                                        372,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            374,
                                            380,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU8Record",
                        ),
                        span: GTSpan(
                            383,
                            413,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                383,
                                394,
                            ),
                            "IntU8Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    397,
                                    413,
                                ),
                                key: IntU8(
                                    GTSpan(
                                        399,
                                        403,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            405,
                                            411,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU16Record",
                        ),
                        span: GTSpan(
                            414,
                            446,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                414,
                                426,
                            ),
                            "IntU16Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    429,
                                    446,
                                ),
                                key: IntU16(
                                    GTSpan(
                                        431,
                                        436,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            438,
                                            444,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU32Record",
                        ),
                        span: GTSpan(
                            447,
                            479,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                447,
                                459,
                            ),
                            "IntU32Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    462,
                                    479,
                                ),
                                key: IntU32(
                                    GTSpan(
                                        464,
                                        469,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            471,
                                            477,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU64Record",
                        ),
                        span: GTSpan(
                            480,
                            512,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                480,
                                492,
                            ),
                            "IntU64Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    495,
                                    512,
                                ),
                                key: IntU64(
                                    GTSpan(
                                        497,
                                        502,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            504,
                                            510,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntU128Record",
                        ),
                        span: GTSpan(
                            513,
                            547,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                513,
                                526,
                            ),
                            "IntU128Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    529,
                                    547,
                                ),
                                key: IntU128(
                                    GTSpan(
                                        531,
                                        537,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            539,
                                            545,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "IntUSizeRecord",
                        ),
                        span: GTSpan(
                            548,
                            584,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                548,
                                562,
                            ),
                            "IntUSizeRecord",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    565,
                                    584,
                                ),
                                key: IntUSize(
                                    GTSpan(
                                        567,
                                        574,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            576,
                                            582,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float32Record",
                        ),
                        span: GTSpan(
                            585,
                            618,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                585,
                                598,
                            ),
                            "Float32Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    601,
                                    618,
                                ),
                                key: Float32(
                                    GTSpan(
                                        603,
                                        608,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            610,
                                            616,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Float64Record",
                        ),
                        span: GTSpan(
                            619,
                            652,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                619,
                                632,
                            ),
                            "Float64Record",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    635,
                                    652,
                                ),
                                key: Float64(
                                    GTSpan(
                                        637,
                                        642,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            644,
                                            650,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            4,
                        ),
                        "Int8",
                    ),
                    GTIdentifier(
                        GTSpan(
                            10,
                            15,
                        ),
                        "Int16",
                    ),
                    GTIdentifier(
                        GTSpan(
                            22,
                            27,
                        ),
                        "Int32",
                    ),
                    GTIdentifier(
                        GTSpan(
                            34,
                            39,
                        ),
                        "Int64",
                    ),
                    GTIdentifier(
                        GTSpan(
                            46,
                            52,
                        ),
                        "Int128",
                    ),
                    GTIdentifier(
                        GTSpan(
                            60,
                            67,
                        ),
                        "IntSize",
                    ),
                    GTIdentifier(
                        GTSpan(
                            76,
                            81,
                        ),
                        "IntU8",
                    ),
                    GTIdentifier(
                        GTSpan(
                            87,
                            93,
                        ),
                        "IntU16",
                    ),
                    GTIdentifier(
                        GTSpan(
                            100,
                            106,
                        ),
                        "IntU32",
                    ),
                    GTIdentifier(
                        GTSpan(
                            113,
                            119,
                        ),
                        "IntU64",
                    ),
                    GTIdentifier(
                        GTSpan(
                            126,
                            133,
                        ),
                        "IntU128",
                    ),
                    GTIdentifier(
                        GTSpan(
                            141,
                            149,
                        ),
                        "IntUSize",
                    ),
                    GTIdentifier(
                        GTSpan(
                            158,
                            165,
                        ),
                        "Float32",
                    ),
                    GTIdentifier(
                        GTSpan(
                            172,
                            179,
                        ),
                        "Float64",
                    ),
                    GTIdentifier(
                        GTSpan(
                            187,
                            197,
                        ),
                        "Int8Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            217,
                            228,
                        ),
                        "Int16Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            249,
                            260,
                        ),
                        "Int32Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            281,
                            292,
                        ),
                        "Int64Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            313,
                            325,
                        ),
                        "Int128Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            347,
                            360,
                        ),
                        "IntSizeRecord",
                    ),
                    GTIdentifier(
                        GTSpan(
                            383,
                            394,
                        ),
                        "IntU8Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            414,
                            426,
                        ),
                        "IntU16Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            447,
                            459,
                        ),
                        "IntU32Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            480,
                            492,
                        ),
                        "IntU64Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            513,
                            526,
                        ),
                        "IntU128Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            548,
                            562,
                        ),
                        "IntUSizeRecord",
                    ),
                    GTIdentifier(
                        GTSpan(
                            585,
                            598,
                        ),
                        "Float32Record",
                    ),
                    GTIdentifier(
                        GTSpan(
                            619,
                            632,
                        ),
                        "Float64Record",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/17-number_sizes.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    #[test]
    fn test_number() {
        assert_debug_snapshot!(parse_module("../examples/02-syntax/18-number.type"), @r#"
        GTModuleParse {
            module: GTModule {
                id: GTModuleId(
                    "module",
                ),
                doc: None,
                imports: [],
                aliases: [
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "Hello",
                        ),
                        span: GTSpan(
                            0,
                            14,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                0,
                                5,
                            ),
                            "Hello",
                        ),
                        descriptor: Primitive(
                            Number(
                                GTSpan(
                                    8,
                                    14,
                                ),
                            ),
                        ),
                    },
                    GTAlias {
                        id: GTDefinitionId(
                            GTModuleId(
                                "module",
                            ),
                            "World",
                        ),
                        span: GTSpan(
                            16,
                            44,
                        ),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(
                            GTSpan(
                                16,
                                21,
                            ),
                            "World",
                        ),
                        descriptor: Record(
                            GTRecord {
                                span: GTSpan(
                                    24,
                                    44,
                                ),
                                key: Number(
                                    GTSpan(
                                        26,
                                        34,
                                    ),
                                ),
                                descriptor: Primitive(
                                    String(
                                        GTSpan(
                                            36,
                                            42,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            resolve: GTModuleResolve {
                deps: {},
                exports: [
                    GTIdentifier(
                        GTSpan(
                            0,
                            5,
                        ),
                        "Hello",
                    ),
                    GTIdentifier(
                        GTSpan(
                            16,
                            21,
                        ),
                        "World",
                    ),
                ],
                references: {},
            },
            source_code: NamedSource {
                name: "../examples/02-syntax/18-number.type",
                source: "<redacted>",
                language: None,
            ,
        }
        "#);
    }

    fn parse_module(path: &str) -> GTModuleParse {
        let content = fs::read_to_string(path).expect("cannot read file");
        let source_code = NamedSource::new(path, content);
        GTModule::parse("module".into(), source_code).unwrap()
    }
}
