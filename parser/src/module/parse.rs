use miette::{NamedSource, Result};
use pest::iterators::Pair;
use serde::Serialize;

use crate::*;

/// Module parse result. It contains the module tree and resolve data.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTModuleParse {
    /// Module tree.
    pub module: GTModule,
    /// Module resolve. It contains module meta information used to build
    /// the dependency graph.
    pub resolve: GTModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[serde(serialize_with = "crate::miette_serde::serialize_named_source")]
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
    use insta::assert_ron_snapshot;
    use miette::NamedSource;
    use std::fs;

    use super::*;

    #[test]
    fn test_alias() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/01-alias.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Age"),
                span: GTSpan(0, 8),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 3), "Age"),
                descriptor: Primitive(Int64(GTSpan(5, 8))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "AnotherAge"),
                span: GTSpan(10, 25),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(10, 20), "AnotherAge"),
                descriptor: Reference(GTReference(
                  span: GTSpan(22, 25),
                  id: GTReferenceId(GTModuleId("module"), GTSpan(22, 25)),
                  definition_id: Unresolved,
                  identifier: GTIdentifier(GTSpan(22, 25), "Age"),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "snake_case"),
                span: GTSpan(27, 42),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(27, 37), "snake_case"),
                descriptor: Primitive(Int64(GTSpan(39, 42))),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 3), "Age"),
              GTIdentifier(GTSpan(10, 20), "AnotherAge"),
              GTIdentifier(GTSpan(27, 37), "snake_case"),
            ],
            references: [
              GTIdentifier(GTSpan(22, 25), "Age"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/01-alias.type",
            source: "Age: int\n\nAnotherAge: Age\n\nsnake_case: int",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_primitives() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/02-primitives.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "String"),
                span: GTSpan(0, 14),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 6), "String"),
                descriptor: Primitive(String(GTSpan(8, 14))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int"),
                span: GTSpan(16, 24),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(16, 19), "Int"),
                descriptor: Primitive(Int64(GTSpan(21, 24))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float"),
                span: GTSpan(26, 38),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(26, 31), "Float"),
                descriptor: Primitive(Float64(GTSpan(33, 38))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Boolean"),
                span: GTSpan(40, 56),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(40, 47), "Boolean"),
                descriptor: Primitive(Boolean(GTSpan(49, 56))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Null"),
                span: GTSpan(58, 68),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(58, 62), "Null"),
                descriptor: Primitive(Null(GTSpan(64, 68))),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 6), "String"),
              GTIdentifier(GTSpan(16, 19), "Int"),
              GTIdentifier(GTSpan(26, 31), "Float"),
              GTIdentifier(GTSpan(40, 47), "Boolean"),
              GTIdentifier(GTSpan(58, 62), "Null"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/02-primitives.type",
            source: "String: string\n\nInt: int\n\nFloat: float\n\nBoolean: boolean\n\nNull: null",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_objects() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/03-objects.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(0, 25),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(7, 25),
                  name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(11, 23),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(11, 15), "name"),
                      descriptor: Primitive(String(GTSpan(17, 23))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(27, 79),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(27, 32), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(34, 79),
                  name: Named(GTIdentifier(GTSpan(27, 32), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(38, 50),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(38, 42), "name"),
                      descriptor: Primitive(String(GTSpan(44, 50))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(53, 61),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(53, 56), "age"),
                      descriptor: Primitive(Int64(GTSpan(58, 61))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(64, 77),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(64, 68), "flag"),
                      descriptor: Primitive(Boolean(GTSpan(70, 77))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Empty"),
                span: GTSpan(81, 90),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(81, 86), "Empty"),
                descriptor: Object(GTObject(
                  span: GTSpan(88, 90),
                  name: Named(GTIdentifier(GTSpan(81, 86), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Empty"),
                span: GTSpan(92, 103),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(92, 97), "Empty"),
                descriptor: Object(GTObject(
                  span: GTSpan(99, 103),
                  name: Named(GTIdentifier(GTSpan(92, 97), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(105, 128),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(105, 110), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(112, 128),
                  name: Named(GTIdentifier(GTSpan(105, 110), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(114, 126),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(114, 118), "name"),
                      descriptor: Primitive(String(GTSpan(120, 126))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(130, 163),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(130, 135), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(137, 163),
                  name: Named(GTIdentifier(GTSpan(130, 135), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(139, 151),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(139, 143), "name"),
                      descriptor: Primitive(String(GTSpan(145, 151))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(153, 161),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(153, 156), "age"),
                      descriptor: Primitive(Int64(GTSpan(158, 161))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "PascalCase"),
                span: GTSpan(165, 198),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(165, 175), "PascalCase"),
                descriptor: Object(GTObject(
                  span: GTSpan(177, 198),
                  name: Named(GTIdentifier(GTSpan(165, 175), "PascalCase")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(181, 196),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(181, 191), "snake_case"),
                      descriptor: Primitive(Int64(GTSpan(193, 196))),
                      required: true,
                    ),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "Hello"),
              GTIdentifier(GTSpan(27, 32), "Hello"),
              GTIdentifier(GTSpan(81, 86), "Empty"),
              GTIdentifier(GTSpan(92, 97), "Empty"),
              GTIdentifier(GTSpan(105, 110), "Hello"),
              GTIdentifier(GTSpan(130, 135), "Hello"),
              GTIdentifier(GTSpan(165, 175), "PascalCase"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/03-objects.type",
            source: "Hello: {\n  name: string\n}\n\nHello: {\n  name: string\n  age: int\n  flag: boolean\n}\n\nEmpty: {}\n\nEmpty: {\n\n}\n\nHello: { name: string }\n\nHello: { name: string, age: int }\n\nPascalCase: {\n  snake_case: int\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_comments() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/04-comments.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: Some(GTDoc(GTSpan(4, 38), "Module comment...\n...multiline")),
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(58, 110),
                doc: Some(GTDoc(GTSpan(62, 75), "Alias comment")),
                attributes: [],
                name: GTIdentifier(GTSpan(76, 81), "Hello"),
                descriptor: Primitive(String(GTSpan(104, 110))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(112, 254),
                doc: Some(GTDoc(GTSpan(116, 149), "Multiline...\n...alias comment")),
                attributes: [],
                name: GTIdentifier(GTSpan(150, 155), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(157, 254),
                  name: Named(GTIdentifier(GTSpan(150, 155), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(161, 196),
                      doc: Some(GTDoc(GTSpan(165, 181), "Property comment")),
                      attributes: [],
                      name: GTKey(GTSpan(184, 188), "name"),
                      descriptor: Primitive(String(GTSpan(190, 196))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(199, 252),
                      doc: Some(GTDoc(GTSpan(203, 241), "Multiline...\n...property comment")),
                      attributes: [],
                      name: GTKey(GTSpan(244, 247), "age"),
                      descriptor: Primitive(Int64(GTSpan(249, 252))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(256, 269),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(256, 261), "Hello"),
                descriptor: Primitive(String(GTSpan(263, 269))),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(76, 81), "Hello"),
              GTIdentifier(GTSpan(150, 155), "Hello"),
              GTIdentifier(GTSpan(256, 261), "Hello"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/04-comments.type",
            source: "//! Module comment...\n//! ...multiline\n\n// Basic comment\n\n/// Alias comment\nHello: /* Inline comment */ string\n\n/// Multiline...\n/// ...alias comment\nHello: {\n  /// Property comment\n  name: string\n  /// Multiline...\n  /// ...property comment\n  age: int\n}\n\nHello: string // Trailing comment",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_optional() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/05-optional.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(0, 37),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(7, 37),
                  name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(11, 23),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(11, 15), "name"),
                      descriptor: Primitive(String(GTSpan(17, 23))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(26, 35),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(26, 29), "age"),
                      descriptor: Primitive(Int64(GTSpan(32, 35))),
                      required: false,
                    ),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "Hello"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/05-optional.type",
            source: "Hello: {\n  name: string\n  age?: int\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_nested() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/06-nested.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(0, 59),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(7, 59),
                  name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(11, 57),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(11, 15), "name"),
                      descriptor: Object(GTObject(
                        span: GTSpan(17, 57),
                        name: Alias(GTIdentifier(GTSpan(17, 57), "HelloName"), Property(GTIdentifier(GTSpan(0, 5), "Hello"), [
                          GTKey(GTSpan(11, 15), "name"),
                        ])),
                        extensions: [],
                        properties: [
                          GTProperty(
                            span: GTSpan(23, 36),
                            doc: None,
                            attributes: [],
                            name: GTKey(GTSpan(23, 28), "first"),
                            descriptor: Primitive(String(GTSpan(30, 36))),
                            required: true,
                          ),
                          GTProperty(
                            span: GTSpan(41, 53),
                            doc: None,
                            attributes: [],
                            name: GTKey(GTSpan(41, 45), "last"),
                            descriptor: Primitive(String(GTSpan(47, 53))),
                            required: true,
                          ),
                        ],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(61, 127),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(61, 66), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(68, 127),
                  name: Named(GTIdentifier(GTSpan(61, 66), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(72, 125),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(72, 76), "name"),
                      descriptor: Alias(GTAlias(
                        id: GTDefinitionId(GTModuleId("module"), "Named"),
                        span: GTSpan(78, 125),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(GTSpan(78, 83), "Named"),
                        descriptor: Object(GTObject(
                          span: GTSpan(85, 125),
                          name: Named(GTIdentifier(GTSpan(78, 83), "Named")),
                          extensions: [],
                          properties: [
                            GTProperty(
                              span: GTSpan(91, 104),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(91, 96), "first"),
                              descriptor: Primitive(String(GTSpan(98, 104))),
                              required: true,
                            ),
                            GTProperty(
                              span: GTSpan(109, 121),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(109, 113), "last"),
                              descriptor: Primitive(String(GTSpan(115, 121))),
                              required: true,
                            ),
                          ],
                        )),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "Hello"),
              GTIdentifier(GTSpan(61, 66), "Hello"),
              GTIdentifier(GTSpan(78, 83), "Named"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/06-nested.type",
            source: "Hello: {\n  name: {\n    first: string\n    last: string\n  }\n}\n\nHello: {\n  name: Named: {\n    first: string\n    last: string\n  }\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_arrays() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/07-arrays.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Book"),
                span: GTSpan(0, 42),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Book"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 42),
                  name: Named(GTIdentifier(GTSpan(0, 4), "Book")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(10, 23),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(10, 15), "title"),
                      descriptor: Primitive(String(GTSpan(17, 23))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(26, 40),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(26, 30), "tags"),
                      descriptor: Array(GTArray(
                        span: GTSpan(32, 40),
                        descriptor: Primitive(String(GTSpan(33, 39))),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "Book"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/07-arrays.type",
            source: "Book: {\n  title: string\n  tags: [string]\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_tuples() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/08-tuples.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "User"),
                span: GTSpan(0, 67),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "User"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 67),
                  name: Named(GTIdentifier(GTSpan(0, 4), "User")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(10, 32),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(10, 14), "name"),
                      descriptor: Tuple(GTTuple(
                        span: GTSpan(16, 32),
                        descriptors: [
                          Primitive(String(GTSpan(17, 23))),
                          Primitive(String(GTSpan(25, 31))),
                        ],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(35, 65),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(35, 42), "address"),
                      descriptor: Tuple(GTTuple(
                        span: GTSpan(44, 65),
                        descriptors: [
                          Primitive(Int64(GTSpan(45, 48))),
                          Primitive(String(GTSpan(50, 56))),
                          Primitive(String(GTSpan(58, 64))),
                        ],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Address"),
                span: GTSpan(69, 99),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(69, 76), "Address"),
                descriptor: Tuple(GTTuple(
                  span: GTSpan(78, 99),
                  descriptors: [
                    Primitive(Int64(GTSpan(79, 82))),
                    Primitive(String(GTSpan(84, 90))),
                    Primitive(String(GTSpan(92, 98))),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "User"),
              GTIdentifier(GTSpan(69, 76), "Address"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/08-tuples.type",
            source: "User: {\n  name: (string, string)\n  address: (int, string, string)\n}\n\nAddress: (int, string, string)",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_modules() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/09-modules.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [
              GTImport(
                span: GTSpan(0, 12),
                path: GTPath(GTSpan(4, 10), Unresolved, "author"),
                reference: Glob(GTSpan(11, 12)),
              ),
              GTImport(
                span: GTSpan(13, 64),
                path: GTPath(GTSpan(17, 29), Unresolved, "../../author"),
                reference: Names(GTSpan(30, 64), [
                  Name(GTSpan(31, 37), GTIdentifier(GTSpan(31, 37), "Author")),
                  Name(GTSpan(39, 44), GTIdentifier(GTSpan(39, 44), "Genre")),
                  Alias(GTSpan(46, 63), GTIdentifier(GTSpan(46, 55), "Something"), GTIdentifier(GTSpan(59, 63), "Else")),
                ]),
              ),
              GTImport(
                span: GTSpan(65, 82),
                path: GTPath(GTSpan(69, 75), Unresolved, "author"),
                reference: Name(GTSpan(76, 82), GTIdentifier(GTSpan(76, 82), "Author")),
              ),
            ],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Book"),
                span: GTSpan(84, 154),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(84, 88), "Book"),
                descriptor: Object(GTObject(
                  span: GTSpan(90, 154),
                  name: Named(GTIdentifier(GTSpan(84, 88), "Book")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(94, 107),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(94, 99), "title"),
                      descriptor: Primitive(String(GTSpan(101, 107))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(110, 137),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(110, 116), "author"),
                      descriptor: InlineImport(GTInlineImport(
                        span: GTSpan(118, 137),
                        name: GTIdentifier(GTSpan(131, 137), "Author"),
                        path: GTPath(GTSpan(118, 130), Unresolved, "../../author"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(140, 152),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(140, 145), "genre"),
                      descriptor: Reference(GTReference(
                        span: GTSpan(147, 152),
                        id: GTReferenceId(GTModuleId("module"), GTSpan(147, 152)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(147, 152), "Genre"),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Author"),
                span: GTSpan(156, 183),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(156, 162), "Author"),
                descriptor: InlineImport(GTInlineImport(
                  span: GTSpan(164, 183),
                  name: GTIdentifier(GTSpan(177, 183), "Author"),
                  path: GTPath(GTSpan(164, 176), Unresolved, "../../author"),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Authors"),
                span: GTSpan(185, 215),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(185, 192), "Authors"),
                descriptor: Array(GTArray(
                  span: GTSpan(194, 215),
                  descriptor: InlineImport(GTInlineImport(
                    span: GTSpan(195, 214),
                    name: GTIdentifier(GTSpan(208, 214), "Author"),
                    path: GTPath(GTSpan(195, 207), Unresolved, "../../author"),
                  )),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [
              GTPath(GTSpan(4, 10), Unresolved, "author"),
              GTPath(GTSpan(17, 29), Unresolved, "../../author"),
              GTPath(GTSpan(69, 75), Unresolved, "author"),
              GTPath(GTSpan(118, 130), Unresolved, "../../author"),
              GTPath(GTSpan(164, 176), Unresolved, "../../author"),
              GTPath(GTSpan(195, 207), Unresolved, "../../author"),
            ],
            exports: [
              GTIdentifier(GTSpan(84, 88), "Book"),
              GTIdentifier(GTSpan(156, 162), "Author"),
              GTIdentifier(GTSpan(185, 192), "Authors"),
            ],
            references: [
              GTIdentifier(GTSpan(147, 152), "Genre"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/09-modules.type",
            source: "use author/*\nuse ../../author/{Author, Genre, Something as Else}\nuse author/Author\n\nBook: {\n  title: string\n  author: ../../author/Author\n  genre: Genre\n}\n\nAuthor: ../../author/Author\n\nAuthors: [../../author/Author]",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_extensions() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/10-extensions.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Base"),
                span: GTSpan(0, 36),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Base"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 36),
                  name: Named(GTIdentifier(GTSpan(0, 4), "Base")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(10, 22),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(10, 14), "name"),
                      descriptor: Primitive(String(GTSpan(16, 22))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(26, 34),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(26, 29), "age"),
                      descriptor: Primitive(Int64(GTSpan(31, 34))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Processor"),
                span: GTSpan(38, 76),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(38, 47), "Processor"),
                descriptor: Object(GTObject(
                  span: GTSpan(49, 76),
                  name: Named(GTIdentifier(GTSpan(38, 47), "Processor")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(53, 60),
                      reference: GTReference(
                        span: GTSpan(56, 60),
                        id: GTReferenceId(GTModuleId("module"), GTSpan(56, 60)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(56, 60), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(64, 74),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(64, 69), "cores"),
                      descriptor: Primitive(Int64(GTSpan(71, 74))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "User"),
                span: GTSpan(78, 114),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(78, 82), "User"),
                descriptor: Object(GTObject(
                  span: GTSpan(84, 114),
                  name: Named(GTIdentifier(GTSpan(78, 82), "User")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(88, 95),
                      reference: GTReference(
                        span: GTSpan(91, 95),
                        id: GTReferenceId(GTModuleId("module"), GTSpan(91, 95)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(91, 95), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(99, 112),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(99, 104), "email"),
                      descriptor: Primitive(String(GTSpan(106, 112))),
                      required: true,
                    ),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "Base"),
              GTIdentifier(GTSpan(38, 47), "Processor"),
              GTIdentifier(GTSpan(78, 82), "User"),
            ],
            references: [
              GTIdentifier(GTSpan(56, 60), "Base"),
              GTIdentifier(GTSpan(91, 95), "Base"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/10-extensions.type",
            source: "Base: {\n  name: string,\n  age: int\n}\n\nProcessor: {\n  ...Base,\n  cores: int\n}\n\nUser: {\n  ...Base,\n  email: string\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_literals() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/11-literals.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "CommentBase"),
                span: GTSpan(0, 38),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 11), "CommentBase"),
                descriptor: Object(GTObject(
                  span: GTSpan(13, 38),
                  name: Named(GTIdentifier(GTSpan(0, 11), "CommentBase")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(17, 21),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(17, 18), "v"),
                      descriptor: Literal(Integer(GTSpan(20, 21), 2)),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(24, 36),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(24, 28), "text"),
                      descriptor: Primitive(String(GTSpan(30, 36))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "UserComment"),
                span: GTSpan(40, 126),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(40, 51), "UserComment"),
                descriptor: Object(GTObject(
                  span: GTSpan(53, 126),
                  name: Named(GTIdentifier(GTSpan(40, 51), "UserComment")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(57, 71),
                      reference: GTReference(
                        span: GTSpan(60, 71),
                        id: GTReferenceId(GTModuleId("module"), GTSpan(60, 71)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(60, 71), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(74, 86),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(74, 78), "type"),
                      descriptor: Literal(String(GTSpan(80, 86), "user")),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(89, 103),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(89, 95), "userId"),
                      descriptor: Primitive(String(GTSpan(97, 103))),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(106, 124),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(106, 115), "published"),
                      descriptor: Primitive(Boolean(GTSpan(117, 124))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "SystemComment"),
                span: GTSpan(128, 198),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(128, 141), "SystemComment"),
                descriptor: Object(GTObject(
                  span: GTSpan(143, 198),
                  name: Named(GTIdentifier(GTSpan(128, 141), "SystemComment")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(147, 161),
                      reference: GTReference(
                        span: GTSpan(150, 161),
                        id: GTReferenceId(GTModuleId("module"), GTSpan(150, 161)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(150, 161), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(164, 178),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(164, 168), "type"),
                      descriptor: Literal(String(GTSpan(170, 178), "system")),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(181, 196),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(181, 190), "published"),
                      descriptor: Literal(Boolean(GTSpan(192, 196), true)),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "False"),
                span: GTSpan(200, 212),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(200, 205), "False"),
                descriptor: Literal(Boolean(GTSpan(207, 212), false)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float"),
                span: GTSpan(214, 230),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(214, 219), "Float"),
                descriptor: Literal(Float(GTSpan(221, 230), 1.000123)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Number"),
                span: GTSpan(232, 249),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(232, 238), "Number"),
                descriptor: Literal(Integer(GTSpan(240, 249), 1234567)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "String"),
                span: GTSpan(251, 281),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(251, 257), "String"),
                descriptor: Literal(String(GTSpan(259, 281), "Hello, \\\"world\\\"! \\\\")),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "NegativeInt"),
                span: GTSpan(283, 298),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(283, 294), "NegativeInt"),
                descriptor: Literal(Integer(GTSpan(296, 298), -1)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "NegativeFloat"),
                span: GTSpan(300, 319),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(300, 313), "NegativeFloat"),
                descriptor: Literal(Float(GTSpan(315, 319), -1.0)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "LargeFloat"),
                span: GTSpan(321, 336),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(321, 331), "LargeFloat"),
                descriptor: Literal(Float(GTSpan(333, 336), 1000000.0)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "SmallFloat"),
                span: GTSpan(338, 356),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(338, 348), "SmallFloat"),
                descriptor: Literal(Float(GTSpan(350, 356), 0.00035)),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 11), "CommentBase"),
              GTIdentifier(GTSpan(40, 51), "UserComment"),
              GTIdentifier(GTSpan(128, 141), "SystemComment"),
              GTIdentifier(GTSpan(200, 205), "False"),
              GTIdentifier(GTSpan(214, 219), "Float"),
              GTIdentifier(GTSpan(232, 238), "Number"),
              GTIdentifier(GTSpan(251, 257), "String"),
              GTIdentifier(GTSpan(283, 294), "NegativeInt"),
              GTIdentifier(GTSpan(300, 313), "NegativeFloat"),
              GTIdentifier(GTSpan(321, 331), "LargeFloat"),
              GTIdentifier(GTSpan(338, 348), "SmallFloat"),
            ],
            references: [
              GTIdentifier(GTSpan(60, 71), "CommentBase"),
              GTIdentifier(GTSpan(150, 161), "CommentBase"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/11-literals.type",
            source: "CommentBase: {\n  v: 2\n  text: string\n}\n\nUserComment: {\n  ...CommentBase\n  type: \"user\"\n  userId: string\n  published: boolean\n}\n\nSystemComment: {\n  ...CommentBase\n  type: \"system\"\n  published: true\n}\n\nFalse: false\n\nFloat: 1.000_123\n\nNumber: 1_234_567\n\nString: \"Hello, \\\"world\\\"! \\\\\"\n\nNegativeInt: -1\n\nNegativeFloat: -1.0\n\nLargeFloat: 1e6\n\nSmallFloat: 3.5e-4\n",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_unions() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/12-unions.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(0, 24),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Union(GTUnion(
                  span: GTSpan(7, 24),
                  descriptors: [
                    Literal(String(GTSpan(7, 14), "Sasha")),
                    Literal(String(GTSpan(17, 24), "world")),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Multiline"),
                span: GTSpan(26, 59),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(26, 35), "Multiline"),
                descriptor: Union(GTUnion(
                  span: GTSpan(39, 59),
                  descriptors: [
                    Literal(String(GTSpan(41, 48), "Hello")),
                    Primitive(String(GTSpan(53, 59))),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "WithComments"),
                span: GTSpan(61, 147),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(61, 73), "WithComments"),
                descriptor: Union(GTUnion(
                  span: GTSpan(100, 147),
                  descriptors: [
                    Literal(String(GTSpan(102, 109), "Hello")),
                    Primitive(String(GTSpan(141, 147))),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "Hello"),
              GTIdentifier(GTSpan(26, 35), "Multiline"),
              GTIdentifier(GTSpan(61, 73), "WithComments"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/12-unions.type",
            source: "Hello: \"Sasha\" | \"world\"\n\nMultiline:\n  | \"Hello\"\n  | string\n\nWithComments:\n  // This is a comment\n  | \"Hello\"\n  // This is a comment too\n  | string",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_attributes() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/13-attributes.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Message"),
                span: GTSpan(0, 19),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 7), "Message"),
                descriptor: Union(GTUnion(
                  span: GTSpan(9, 19),
                  descriptors: [
                    Reference(GTReference(
                      span: GTSpan(9, 14),
                      id: GTReferenceId(GTModuleId("module"), GTSpan(9, 14)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(9, 14), "Reply"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(17, 19),
                      id: GTReferenceId(GTModuleId("module"), GTSpan(17, 19)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(17, 19), "DM"),
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Reply"),
                span: GTSpan(21, 75),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(21, 26), "Reply"),
                descriptor: Object(GTObject(
                  span: GTSpan(28, 75),
                  name: Named(GTIdentifier(GTSpan(21, 26), "Reply")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(32, 54),
                      doc: None,
                      attributes: [
                        GTAttribute(
                          span: GTSpan(32, 38),
                          name: GTAttributeName(
                            span: GTSpan(34, 37),
                            name: "tag",
                          ),
                          descriptor: None,
                        ),
                      ],
                      name: GTKey(GTSpan(41, 45), "type"),
                      descriptor: Literal(String(GTSpan(47, 54), "reply")),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(58, 73),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(58, 65), "message"),
                      descriptor: Primitive(String(GTSpan(67, 73))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "DM"),
                span: GTSpan(77, 125),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(77, 79), "DM"),
                descriptor: Object(GTObject(
                  span: GTSpan(81, 125),
                  name: Named(GTIdentifier(GTSpan(77, 79), "DM")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(85, 104),
                      doc: None,
                      attributes: [
                        GTAttribute(
                          span: GTSpan(85, 91),
                          name: GTAttributeName(
                            span: GTSpan(87, 90),
                            name: "tag",
                          ),
                          descriptor: None,
                        ),
                      ],
                      name: GTKey(GTSpan(94, 98), "type"),
                      descriptor: Literal(String(GTSpan(100, 104), "dm")),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(108, 123),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(108, 115), "message"),
                      descriptor: Primitive(String(GTSpan(117, 123))),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Assignment"),
                span: GTSpan(127, 161),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(127, 145),
                    name: GTAttributeName(
                      span: GTSpan(129, 134),
                      name: "hello",
                    ),
                    descriptor: Some(Assignment(GTAttributeAssignment(
                      span: GTSpan(135, 144),
                      value: Literal(String(GTSpan(137, 144), "world")),
                    ))),
                  ),
                ],
                name: GTIdentifier(GTSpan(146, 156), "Assignment"),
                descriptor: Literal(Integer(GTSpan(158, 161), 123)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Arguments"),
                span: GTSpan(163, 205),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(163, 189),
                    name: GTAttributeName(
                      span: GTSpan(165, 170),
                      name: "hello",
                    ),
                    descriptor: Some(Arguments([
                      Literal(String(GTSpan(171, 178), "cruel")),
                      Literal(String(GTSpan(180, 187), "world")),
                    ])),
                  ),
                ],
                name: GTIdentifier(GTSpan(190, 199), "Arguments"),
                descriptor: Literal(Boolean(GTSpan(201, 205), true)),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Properties"),
                span: GTSpan(207, 265),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(207, 248),
                    name: GTAttributeName(
                      span: GTSpan(209, 214),
                      name: "hello",
                    ),
                    descriptor: Some(Properties([
                      GTAttributeProperty(
                        span: GTSpan(215, 230),
                        name: GTAttributeKey(
                          span: GTSpan(215, 220),
                          name: "which",
                        ),
                        value: Literal(String(GTSpan(223, 230), "cruel")),
                      ),
                      GTAttributeProperty(
                        span: GTSpan(232, 246),
                        name: GTAttributeKey(
                          span: GTSpan(232, 236),
                          name: "what",
                        ),
                        value: Literal(String(GTSpan(239, 246), "world")),
                      ),
                    ])),
                  ),
                ],
                name: GTIdentifier(GTSpan(249, 259), "Properties"),
                descriptor: Literal(Boolean(GTSpan(261, 265), true)),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 7), "Message"),
              GTIdentifier(GTSpan(21, 26), "Reply"),
              GTIdentifier(GTSpan(77, 79), "DM"),
              GTIdentifier(GTSpan(146, 156), "Assignment"),
              GTIdentifier(GTSpan(190, 199), "Arguments"),
              GTIdentifier(GTSpan(249, 259), "Properties"),
            ],
            references: [
              GTIdentifier(GTSpan(9, 14), "Reply"),
              GTIdentifier(GTSpan(17, 19), "DM"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/13-attributes.type",
            source: "Message: Reply | DM\n\nReply: {\n  #[tag]\n  type: \"reply\",\n  message: string\n}\n\nDM: {\n  #[tag]\n  type: \"dm\",\n  message: string\n}\n\n#[hello = \"world\"]\nAssignment: 123\n\n#[hello(\"cruel\", \"world\")]\nArguments: true\n\n#[hello(which = \"cruel\", what = \"world\")]\nProperties: true",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_records() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/14-records.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Dict"),
                span: GTSpan(0, 20),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Dict"),
                descriptor: Record(GTRecord(
                  span: GTSpan(6, 20),
                  key: String(GTSpan(8, 10)),
                  descriptor: Primitive(String(GTSpan(12, 18))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Map"),
                span: GTSpan(22, 44),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(22, 25), "Map"),
                descriptor: Record(GTRecord(
                  span: GTSpan(27, 44),
                  key: Int64(GTSpan(29, 34)),
                  descriptor: Primitive(String(GTSpan(36, 42))),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "Dict"),
              GTIdentifier(GTSpan(22, 25), "Map"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/14-records.type",
            source: "Dict: { []: string }\n\nMap: { [int]: string }",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_any() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/15-any.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Anything"),
                span: GTSpan(0, 13),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 8), "Anything"),
                descriptor: Any(GTAny(GTSpan(10, 13))),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 8), "Anything"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/15-any.type",
            source: "Anything: any",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_branded() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/16-branded.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "OrgId"),
                span: GTSpan(0, 11),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "OrgId"),
                descriptor: Branded(GTBranded(
                  span: GTSpan(7, 11),
                  id: GTDefinitionId(GTModuleId("module"), "OrgId"),
                  name: GTIdentifier(GTSpan(0, 5), "OrgId"),
                  primitive: Int64(GTSpan(8, 11)),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "UserId"),
                span: GTSpan(13, 28),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(13, 19), "UserId"),
                descriptor: Branded(GTBranded(
                  span: GTSpan(21, 28),
                  id: GTDefinitionId(GTModuleId("module"), "UserId"),
                  name: GTIdentifier(GTSpan(13, 19), "UserId"),
                  primitive: String(GTSpan(22, 28)),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Const"),
                span: GTSpan(30, 43),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(30, 35), "Const"),
                descriptor: Branded(GTBranded(
                  span: GTSpan(37, 43),
                  id: GTDefinitionId(GTModuleId("module"), "Const"),
                  name: GTIdentifier(GTSpan(30, 35), "Const"),
                  primitive: Float64(GTSpan(38, 43)),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Yes"),
                span: GTSpan(45, 58),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(45, 48), "Yes"),
                descriptor: Branded(GTBranded(
                  span: GTSpan(50, 58),
                  id: GTDefinitionId(GTModuleId("module"), "Yes"),
                  name: GTIdentifier(GTSpan(45, 48), "Yes"),
                  primitive: Boolean(GTSpan(51, 58)),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Nope"),
                span: GTSpan(60, 71),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(60, 64), "Nope"),
                descriptor: Branded(GTBranded(
                  span: GTSpan(66, 71),
                  id: GTDefinitionId(GTModuleId("module"), "Nope"),
                  name: GTIdentifier(GTSpan(60, 64), "Nope"),
                  primitive: Null(GTSpan(67, 71)),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "OrgId"),
              GTIdentifier(GTSpan(13, 19), "UserId"),
              GTIdentifier(GTSpan(30, 35), "Const"),
              GTIdentifier(GTSpan(45, 48), "Yes"),
              GTIdentifier(GTSpan(60, 64), "Nope"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/16-branded.type",
            source: "OrgId: @int\n\nUserId: @string\n\nConst: @float\n\nYes: @boolean\n\nNope: @null",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_number_sizes() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/17-number_sizes.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int8"),
                span: GTSpan(0, 8),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Int8"),
                descriptor: Primitive(Int8(GTSpan(6, 8))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int16"),
                span: GTSpan(9, 19),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(9, 14), "Int16"),
                descriptor: Primitive(Int16(GTSpan(16, 19))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int32"),
                span: GTSpan(20, 30),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(20, 25), "Int32"),
                descriptor: Primitive(Int32(GTSpan(27, 30))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int64"),
                span: GTSpan(31, 41),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(31, 36), "Int64"),
                descriptor: Primitive(Int64(GTSpan(38, 41))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int128"),
                span: GTSpan(42, 54),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(42, 48), "Int128"),
                descriptor: Primitive(Int128(GTSpan(50, 54))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntSize"),
                span: GTSpan(55, 69),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(55, 62), "IntSize"),
                descriptor: Primitive(IntSize(GTSpan(64, 69))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU8"),
                span: GTSpan(70, 79),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(70, 75), "IntU8"),
                descriptor: Primitive(IntU8(GTSpan(77, 79))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU16"),
                span: GTSpan(80, 91),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(80, 86), "IntU16"),
                descriptor: Primitive(IntU16(GTSpan(88, 91))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU32"),
                span: GTSpan(92, 103),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(92, 98), "IntU32"),
                descriptor: Primitive(IntU32(GTSpan(100, 103))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU64"),
                span: GTSpan(104, 115),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(104, 110), "IntU64"),
                descriptor: Primitive(IntU64(GTSpan(112, 115))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU128"),
                span: GTSpan(116, 129),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(116, 123), "IntU128"),
                descriptor: Primitive(IntU128(GTSpan(125, 129))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntUSize"),
                span: GTSpan(130, 145),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(130, 138), "IntUSize"),
                descriptor: Primitive(IntUSize(GTSpan(140, 145))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float32"),
                span: GTSpan(146, 158),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(146, 153), "Float32"),
                descriptor: Primitive(Float32(GTSpan(155, 158))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float64"),
                span: GTSpan(159, 171),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(159, 166), "Float64"),
                descriptor: Primitive(Float64(GTSpan(168, 171))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int8Record"),
                span: GTSpan(173, 201),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(173, 183), "Int8Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(185, 201),
                  key: Int8(GTSpan(187, 191)),
                  descriptor: Primitive(String(GTSpan(193, 199))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int16Record"),
                span: GTSpan(202, 232),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(202, 213), "Int16Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(215, 232),
                  key: Int16(GTSpan(217, 222)),
                  descriptor: Primitive(String(GTSpan(224, 230))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int32Record"),
                span: GTSpan(233, 263),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(233, 244), "Int32Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(246, 263),
                  key: Int32(GTSpan(248, 253)),
                  descriptor: Primitive(String(GTSpan(255, 261))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int64Record"),
                span: GTSpan(264, 294),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(264, 275), "Int64Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(277, 294),
                  key: Int64(GTSpan(279, 284)),
                  descriptor: Primitive(String(GTSpan(286, 292))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int128Record"),
                span: GTSpan(295, 327),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(295, 307), "Int128Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(309, 327),
                  key: Int128(GTSpan(311, 317)),
                  descriptor: Primitive(String(GTSpan(319, 325))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntSizeRecord"),
                span: GTSpan(328, 362),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(328, 341), "IntSizeRecord"),
                descriptor: Record(GTRecord(
                  span: GTSpan(343, 362),
                  key: IntSize(GTSpan(345, 352)),
                  descriptor: Primitive(String(GTSpan(354, 360))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU8Record"),
                span: GTSpan(363, 392),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(363, 374), "IntU8Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(376, 392),
                  key: IntU8(GTSpan(378, 382)),
                  descriptor: Primitive(String(GTSpan(384, 390))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU16Record"),
                span: GTSpan(393, 424),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(393, 405), "IntU16Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(407, 424),
                  key: IntU16(GTSpan(409, 414)),
                  descriptor: Primitive(String(GTSpan(416, 422))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU32Record"),
                span: GTSpan(425, 456),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(425, 437), "IntU32Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(439, 456),
                  key: IntU32(GTSpan(441, 446)),
                  descriptor: Primitive(String(GTSpan(448, 454))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU64Record"),
                span: GTSpan(457, 488),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(457, 469), "IntU64Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(471, 488),
                  key: IntU64(GTSpan(473, 478)),
                  descriptor: Primitive(String(GTSpan(480, 486))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU128Record"),
                span: GTSpan(489, 522),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(489, 502), "IntU128Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(504, 522),
                  key: IntU128(GTSpan(506, 512)),
                  descriptor: Primitive(String(GTSpan(514, 520))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntUSizeRecord"),
                span: GTSpan(523, 558),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(523, 537), "IntUSizeRecord"),
                descriptor: Record(GTRecord(
                  span: GTSpan(539, 558),
                  key: IntUSize(GTSpan(541, 548)),
                  descriptor: Primitive(String(GTSpan(550, 556))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float32Record"),
                span: GTSpan(559, 591),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(559, 572), "Float32Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(574, 591),
                  key: Float32(GTSpan(576, 581)),
                  descriptor: Primitive(String(GTSpan(583, 589))),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float64Record"),
                span: GTSpan(592, 624),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(592, 605), "Float64Record"),
                descriptor: Record(GTRecord(
                  span: GTSpan(607, 624),
                  key: Float64(GTSpan(609, 614)),
                  descriptor: Primitive(String(GTSpan(616, 622))),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "Int8"),
              GTIdentifier(GTSpan(9, 14), "Int16"),
              GTIdentifier(GTSpan(20, 25), "Int32"),
              GTIdentifier(GTSpan(31, 36), "Int64"),
              GTIdentifier(GTSpan(42, 48), "Int128"),
              GTIdentifier(GTSpan(55, 62), "IntSize"),
              GTIdentifier(GTSpan(70, 75), "IntU8"),
              GTIdentifier(GTSpan(80, 86), "IntU16"),
              GTIdentifier(GTSpan(92, 98), "IntU32"),
              GTIdentifier(GTSpan(104, 110), "IntU64"),
              GTIdentifier(GTSpan(116, 123), "IntU128"),
              GTIdentifier(GTSpan(130, 138), "IntUSize"),
              GTIdentifier(GTSpan(146, 153), "Float32"),
              GTIdentifier(GTSpan(159, 166), "Float64"),
              GTIdentifier(GTSpan(173, 183), "Int8Record"),
              GTIdentifier(GTSpan(202, 213), "Int16Record"),
              GTIdentifier(GTSpan(233, 244), "Int32Record"),
              GTIdentifier(GTSpan(264, 275), "Int64Record"),
              GTIdentifier(GTSpan(295, 307), "Int128Record"),
              GTIdentifier(GTSpan(328, 341), "IntSizeRecord"),
              GTIdentifier(GTSpan(363, 374), "IntU8Record"),
              GTIdentifier(GTSpan(393, 405), "IntU16Record"),
              GTIdentifier(GTSpan(425, 437), "IntU32Record"),
              GTIdentifier(GTSpan(457, 469), "IntU64Record"),
              GTIdentifier(GTSpan(489, 502), "IntU128Record"),
              GTIdentifier(GTSpan(523, 537), "IntUSizeRecord"),
              GTIdentifier(GTSpan(559, 572), "Float32Record"),
              GTIdentifier(GTSpan(592, 605), "Float64Record"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/17-number_sizes.type",
            source: "Int8: i8\nInt16: i16\nInt32: i32\nInt64: i64\nInt128: i128\nIntSize: isize\nIntU8: u8\nIntU16: u16\nIntU32: u32\nIntU64: u64\nIntU128: u128\nIntUSize: usize\nFloat32: f32\nFloat64: f64\n\nInt8Record: { [i8]: string }\nInt16Record: { [i16]: string }\nInt32Record: { [i32]: string }\nInt64Record: { [i64]: string }\nInt128Record: { [i128]: string }\nIntSizeRecord: { [isize]: string }\nIntU8Record: { [u8]: string }\nIntU16Record: { [u16]: string }\nIntU32Record: { [u32]: string }\nIntU64Record: { [u64]: string }\nIntU128Record: { [u128]: string }\nIntUSizeRecord: { [usize]: string }\nFloat32Record: { [f32]: string }\nFloat64Record: { [f64]: string }\n",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_number() {
        assert_ron_snapshot!(parse_module("../examples/02-syntax/18-number.type"), @r#"
        GTModuleParse(
          module: GTModule(
            id: GTModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(0, 13),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Primitive(Number(GTSpan(7, 13))),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "World"),
                span: GTSpan(15, 42),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(15, 20), "World"),
                descriptor: Record(GTRecord(
                  span: GTSpan(22, 42),
                  key: Number(GTSpan(24, 32)),
                  descriptor: Primitive(String(GTSpan(34, 40))),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 5), "Hello"),
              GTIdentifier(GTSpan(15, 20), "World"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/18-number.type",
            source: "Hello: number\n\nWorld: { [number]: string }",
            language: None,
          ),
        )
        "#);
    }

    fn parse_module(path: &str) -> GTModuleParse {
        let content = fs::read_to_string(path).expect("cannot read file");
        let source_code = NamedSource::new(path, content);
        GTModule::parse("module".into(), source_code).unwrap()
    }
}
