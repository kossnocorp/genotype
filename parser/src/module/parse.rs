use crate::prelude::internal::*;

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

                rule => {
                    return Err(GTParseError::UnexpectedRule(
                        pair.as_span().into(),
                        GTNode::Module,
                        rule,
                    )
                    .into());
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(5, 8),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "AnotherAge"),
                span: GTSpan(10, 25),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(10, 20), "AnotherAge"),
                descriptor: Reference(GTReference(
                  span: GTSpan(22, 25),
                  doc: None,
                  attributes: [],
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(39, 42),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(8, 14),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int"),
                span: GTSpan(16, 24),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(16, 19), "Int"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(21, 24),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float"),
                span: GTSpan(26, 38),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(26, 31), "Float"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(33, 38),
                  kind: Float64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Boolean"),
                span: GTSpan(40, 56),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(40, 47), "Boolean"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(49, 56),
                  kind: Boolean,
                  doc: None,
                  attributes: [],
                )),
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
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/02-primitives.type",
            source: "String: string\n\nInt: int\n\nFloat: float\n\nBoolean: boolean",
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
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(27, 82),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(27, 32), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(34, 82),
                  name: Named(GTIdentifier(GTSpan(27, 32), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(38, 50),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(38, 42), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(44, 50),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(54, 62),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(54, 57), "age"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(59, 62),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(66, 79),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(66, 70), "flag"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(72, 79),
                        kind: Boolean,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Empty"),
                span: GTSpan(84, 93),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(84, 89), "Empty"),
                descriptor: Object(GTObject(
                  span: GTSpan(91, 93),
                  name: Named(GTIdentifier(GTSpan(84, 89), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Empty"),
                span: GTSpan(95, 106),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(95, 100), "Empty"),
                descriptor: Object(GTObject(
                  span: GTSpan(102, 106),
                  name: Named(GTIdentifier(GTSpan(95, 100), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(108, 131),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(108, 113), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(115, 131),
                  name: Named(GTIdentifier(GTSpan(108, 113), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(117, 129),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(117, 121), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(123, 129),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(133, 166),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(133, 138), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(140, 166),
                  name: Named(GTIdentifier(GTSpan(133, 138), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(142, 154),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(142, 146), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(148, 154),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(156, 164),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(156, 159), "age"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(161, 164),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "PascalCase"),
                span: GTSpan(168, 202),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(168, 178), "PascalCase"),
                descriptor: Object(GTObject(
                  span: GTSpan(180, 202),
                  name: Named(GTIdentifier(GTSpan(168, 178), "PascalCase")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(184, 199),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(184, 194), "snake_case"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(196, 199),
                        kind: Int64,
                        doc: None,
                        attributes: [],
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
              GTIdentifier(GTSpan(27, 32), "Hello"),
              GTIdentifier(GTSpan(84, 89), "Empty"),
              GTIdentifier(GTSpan(95, 100), "Empty"),
              GTIdentifier(GTSpan(108, 113), "Hello"),
              GTIdentifier(GTSpan(133, 138), "Hello"),
              GTIdentifier(GTSpan(168, 178), "PascalCase"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/03-objects.type",
            source: "Hello: {\n  name: string\n}\n\nHello: {\n  name: string,\n  age: int,\n  flag: boolean,\n}\n\nEmpty: {}\n\nEmpty: {\n\n}\n\nHello: { name: string }\n\nHello: { name: string, age: int }\n\nPascalCase: {\n  snake_case: int,\n}",
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(104, 110),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(112, 256),
                doc: Some(GTDoc(GTSpan(116, 149), "Multiline...\n...alias comment")),
                attributes: [],
                name: GTIdentifier(GTSpan(150, 155), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(157, 256),
                  name: Named(GTIdentifier(GTSpan(150, 155), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(161, 196),
                      doc: Some(GTDoc(GTSpan(165, 181), "Property comment")),
                      attributes: [],
                      name: GTKey(GTSpan(184, 188), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(190, 196),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(200, 253),
                      doc: Some(GTDoc(GTSpan(204, 242), "Multiline...\n...property comment")),
                      attributes: [],
                      name: GTKey(GTSpan(245, 248), "age"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(250, 253),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Hello"),
                span: GTSpan(258, 271),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(258, 263), "Hello"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(265, 271),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(76, 81), "Hello"),
              GTIdentifier(GTSpan(150, 155), "Hello"),
              GTIdentifier(GTSpan(258, 263), "Hello"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/04-comments.type",
            source: "//! Module comment...\n//! ...multiline\n\n// Basic comment\n\n/// Alias comment\nHello: /* Inline comment */ string\n\n/// Multiline...\n/// ...alias comment\nHello: {\n  /// Property comment\n  name: string,\n  /// Multiline...\n  /// ...property comment\n  age: int,\n}\n\nHello: string // Trailing comment",
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
                span: GTSpan(0, 38),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(7, 38),
                  name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(11, 23),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(11, 15), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(27, 36),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(27, 30), "age"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(33, 36),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
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
            source: "Hello: {\n  name: string,\n  age?: int\n}",
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
                span: GTSpan(0, 61),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 5), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(7, 61),
                  name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(11, 59),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(11, 15), "name"),
                      descriptor: Object(GTObject(
                        span: GTSpan(17, 59),
                        name: Alias(GTIdentifier(GTSpan(17, 59), "HelloName"), Property(GTIdentifier(GTSpan(0, 5), "Hello"), [
                          GTKey(GTSpan(11, 15), "name"),
                        ])),
                        extensions: [],
                        properties: [
                          GTProperty(
                            span: GTSpan(23, 36),
                            doc: None,
                            attributes: [],
                            name: GTKey(GTSpan(23, 28), "first"),
                            descriptor: Primitive(GTPrimitive(
                              span: GTSpan(30, 36),
                              kind: String,
                              doc: None,
                              attributes: [],
                            )),
                            required: true,
                          ),
                          GTProperty(
                            span: GTSpan(42, 54),
                            doc: None,
                            attributes: [],
                            name: GTKey(GTSpan(42, 46), "last"),
                            descriptor: Primitive(GTPrimitive(
                              span: GTSpan(48, 54),
                              kind: String,
                              doc: None,
                              attributes: [],
                            )),
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
                span: GTSpan(63, 131),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(63, 68), "Hello"),
                descriptor: Object(GTObject(
                  span: GTSpan(70, 131),
                  name: Named(GTIdentifier(GTSpan(63, 68), "Hello")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(74, 129),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(74, 78), "name"),
                      descriptor: Alias(GTAlias(
                        id: GTDefinitionId(GTModuleId("module"), "Named"),
                        span: GTSpan(80, 129),
                        doc: None,
                        attributes: [],
                        name: GTIdentifier(GTSpan(80, 85), "Named"),
                        descriptor: Object(GTObject(
                          span: GTSpan(87, 129),
                          name: Named(GTIdentifier(GTSpan(80, 85), "Named")),
                          extensions: [],
                          properties: [
                            GTProperty(
                              span: GTSpan(93, 106),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(93, 98), "first"),
                              descriptor: Primitive(GTPrimitive(
                                span: GTSpan(100, 106),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
                              required: true,
                            ),
                            GTProperty(
                              span: GTSpan(112, 124),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(112, 116), "last"),
                              descriptor: Primitive(GTPrimitive(
                                span: GTSpan(118, 124),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
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
              GTIdentifier(GTSpan(63, 68), "Hello"),
              GTIdentifier(GTSpan(80, 85), "Named"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/06-nested.type",
            source: "Hello: {\n  name: {\n    first: string,\n    last: string,\n  }\n}\n\nHello: {\n  name: Named: {\n    first: string,\n    last: string,\n  }\n}",
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
                span: GTSpan(0, 44),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Book"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 44),
                  name: Named(GTIdentifier(GTSpan(0, 4), "Book")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(10, 23),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(10, 15), "title"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(27, 41),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(27, 31), "tags"),
                      descriptor: Array(GTArray(
                        span: GTSpan(33, 41),
                        descriptor: Primitive(GTPrimitive(
                          span: GTSpan(34, 40),
                          kind: String,
                          doc: None,
                          attributes: [],
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
              GTIdentifier(GTSpan(0, 4), "Book"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/07-arrays.type",
            source: "Book: {\n  title: string,\n  tags: [string],\n}",
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
                span: GTSpan(0, 69),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "User"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 69),
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
                          Primitive(GTPrimitive(
                            span: GTSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GTPrimitive(
                            span: GTSpan(25, 31),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                        ],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(36, 66),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(36, 43), "address"),
                      descriptor: Tuple(GTTuple(
                        span: GTSpan(45, 66),
                        descriptors: [
                          Primitive(GTPrimitive(
                            span: GTSpan(46, 49),
                            kind: Int64,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GTPrimitive(
                            span: GTSpan(51, 57),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GTPrimitive(
                            span: GTSpan(59, 65),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                        ],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Address"),
                span: GTSpan(71, 101),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(71, 78), "Address"),
                descriptor: Tuple(GTTuple(
                  span: GTSpan(80, 101),
                  descriptors: [
                    Primitive(GTPrimitive(
                      span: GTSpan(81, 84),
                      kind: Int64,
                      doc: None,
                      attributes: [],
                    )),
                    Primitive(GTPrimitive(
                      span: GTSpan(86, 92),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                    Primitive(GTPrimitive(
                      span: GTSpan(94, 100),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Empty"),
                span: GTSpan(103, 112),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(103, 108), "Empty"),
                descriptor: Tuple(GTTuple(
                  span: GTSpan(110, 112),
                  descriptors: [],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 4), "User"),
              GTIdentifier(GTSpan(71, 78), "Address"),
              GTIdentifier(GTSpan(103, 108), "Empty"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/08-tuples.type",
            source: "User: {\n  name: (string, string),\n  address: (int, string, string),\n}\n\nAddress: (int, string, string)\n\nEmpty: ()",
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
                span: GTSpan(84, 157),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(84, 88), "Book"),
                descriptor: Object(GTObject(
                  span: GTSpan(90, 157),
                  name: Named(GTIdentifier(GTSpan(84, 88), "Book")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(94, 107),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(94, 99), "title"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(101, 107),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(111, 138),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(111, 117), "author"),
                      descriptor: InlineImport(GTInlineImport(
                        span: GTSpan(119, 138),
                        name: GTIdentifier(GTSpan(132, 138), "Author"),
                        path: GTPath(GTSpan(119, 131), Unresolved, "../../author"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(142, 154),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(142, 147), "genre"),
                      descriptor: Reference(GTReference(
                        span: GTSpan(149, 154),
                        doc: None,
                        attributes: [],
                        id: GTReferenceId(GTModuleId("module"), GTSpan(149, 154)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(149, 154), "Genre"),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Author"),
                span: GTSpan(159, 186),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(159, 165), "Author"),
                descriptor: InlineImport(GTInlineImport(
                  span: GTSpan(167, 186),
                  name: GTIdentifier(GTSpan(180, 186), "Author"),
                  path: GTPath(GTSpan(167, 179), Unresolved, "../../author"),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Authors"),
                span: GTSpan(188, 218),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(188, 195), "Authors"),
                descriptor: Array(GTArray(
                  span: GTSpan(197, 218),
                  descriptor: InlineImport(GTInlineImport(
                    span: GTSpan(198, 217),
                    name: GTIdentifier(GTSpan(211, 217), "Author"),
                    path: GTPath(GTSpan(198, 210), Unresolved, "../../author"),
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
              GTPath(GTSpan(119, 131), Unresolved, "../../author"),
              GTPath(GTSpan(167, 179), Unresolved, "../../author"),
              GTPath(GTSpan(198, 210), Unresolved, "../../author"),
            ],
            exports: [
              GTIdentifier(GTSpan(84, 88), "Book"),
              GTIdentifier(GTSpan(159, 165), "Author"),
              GTIdentifier(GTSpan(188, 195), "Authors"),
            ],
            references: [
              GTIdentifier(GTSpan(149, 154), "Genre"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/09-modules.type",
            source: "use author/*\nuse ../../author/{Author, Genre, Something as Else}\nuse author/Author\n\nBook: {\n  title: string,\n  author: ../../author/Author,\n  genre: Genre,\n}\n\nAuthor: ../../author/Author\n\nAuthors: [../../author/Author]",
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
                span: GTSpan(0, 37),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 4), "Base"),
                descriptor: Object(GTObject(
                  span: GTSpan(6, 37),
                  name: Named(GTIdentifier(GTSpan(0, 4), "Base")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(10, 22),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(10, 14), "name"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(16, 22),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(26, 34),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(26, 29), "age"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(31, 34),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Processor"),
                span: GTSpan(39, 78),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(39, 48), "Processor"),
                descriptor: Object(GTObject(
                  span: GTSpan(50, 78),
                  name: Named(GTIdentifier(GTSpan(39, 48), "Processor")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(54, 61),
                      reference: GTReference(
                        span: GTSpan(57, 61),
                        doc: None,
                        attributes: [],
                        id: GTReferenceId(GTModuleId("module"), GTSpan(57, 61)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(57, 61), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(65, 75),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(65, 70), "cores"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(72, 75),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "User"),
                span: GTSpan(80, 117),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(80, 84), "User"),
                descriptor: Object(GTObject(
                  span: GTSpan(86, 117),
                  name: Named(GTIdentifier(GTSpan(80, 84), "User")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(90, 97),
                      reference: GTReference(
                        span: GTSpan(93, 97),
                        doc: None,
                        attributes: [],
                        id: GTReferenceId(GTModuleId("module"), GTSpan(93, 97)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(93, 97), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(101, 114),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(101, 106), "email"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(108, 114),
                        kind: String,
                        doc: None,
                        attributes: [],
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
              GTIdentifier(GTSpan(0, 4), "Base"),
              GTIdentifier(GTSpan(39, 48), "Processor"),
              GTIdentifier(GTSpan(80, 84), "User"),
            ],
            references: [
              GTIdentifier(GTSpan(57, 61), "Base"),
              GTIdentifier(GTSpan(93, 97), "Base"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/10-extensions.type",
            source: "Base: {\n  name: string,\n  age: int,\n}\n\nProcessor: {\n  ...Base,\n  cores: int,\n}\n\nUser: {\n  ...Base,\n  email: string,\n}",
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
                span: GTSpan(0, 40),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(0, 11), "CommentBase"),
                descriptor: Object(GTObject(
                  span: GTSpan(13, 40),
                  name: Named(GTIdentifier(GTSpan(0, 11), "CommentBase")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(17, 21),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(17, 18), "v"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(20, 21),
                        doc: None,
                        attributes: [],
                        value: Integer(2),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(25, 37),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(25, 29), "text"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(31, 37),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "UserComment"),
                span: GTSpan(42, 132),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(42, 53), "UserComment"),
                descriptor: Object(GTObject(
                  span: GTSpan(55, 132),
                  name: Named(GTIdentifier(GTSpan(42, 53), "UserComment")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(59, 73),
                      reference: GTReference(
                        span: GTSpan(62, 73),
                        doc: None,
                        attributes: [],
                        id: GTReferenceId(GTModuleId("module"), GTSpan(62, 73)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(62, 73), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(77, 89),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(77, 81), "type"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(83, 89),
                        doc: None,
                        attributes: [],
                        value: String("user"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(93, 107),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(93, 99), "userId"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(101, 107),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(111, 129),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(111, 120), "published"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(122, 129),
                        kind: Boolean,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "SystemComment"),
                span: GTSpan(134, 207),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(134, 147), "SystemComment"),
                descriptor: Object(GTObject(
                  span: GTSpan(149, 207),
                  name: Named(GTIdentifier(GTSpan(134, 147), "SystemComment")),
                  extensions: [
                    GTExtension(
                      span: GTSpan(153, 167),
                      reference: GTReference(
                        span: GTSpan(156, 167),
                        doc: None,
                        attributes: [],
                        id: GTReferenceId(GTModuleId("module"), GTSpan(156, 167)),
                        definition_id: Unresolved,
                        identifier: GTIdentifier(GTSpan(156, 167), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GTProperty(
                      span: GTSpan(171, 185),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(171, 175), "type"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(177, 185),
                        doc: None,
                        attributes: [],
                        value: String("system"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(189, 204),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(189, 198), "published"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(200, 204),
                        doc: None,
                        attributes: [],
                        value: Boolean(true),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "False"),
                span: GTSpan(209, 221),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(209, 214), "False"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(216, 221),
                  doc: None,
                  attributes: [],
                  value: Boolean(false),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float"),
                span: GTSpan(223, 239),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(223, 228), "Float"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(230, 239),
                  doc: None,
                  attributes: [],
                  value: Float(1.000123),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Number"),
                span: GTSpan(241, 258),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(241, 247), "Number"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(249, 258),
                  doc: None,
                  attributes: [],
                  value: Integer(1234567),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "String"),
                span: GTSpan(260, 290),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(260, 266), "String"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(268, 290),
                  doc: None,
                  attributes: [],
                  value: String("Hello, \\\"world\\\"! \\\\"),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "NegativeInt"),
                span: GTSpan(292, 307),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(292, 303), "NegativeInt"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(305, 307),
                  doc: None,
                  attributes: [],
                  value: Integer(-1),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "NegativeFloat"),
                span: GTSpan(309, 328),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(309, 322), "NegativeFloat"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(324, 328),
                  doc: None,
                  attributes: [],
                  value: Float(-1.0),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "LargeFloat"),
                span: GTSpan(330, 345),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(330, 340), "LargeFloat"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(342, 345),
                  doc: None,
                  attributes: [],
                  value: Float(1000000.0),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "SmallFloat"),
                span: GTSpan(347, 365),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(347, 357), "SmallFloat"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(359, 365),
                  doc: None,
                  attributes: [],
                  value: Float(0.00035),
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 11), "CommentBase"),
              GTIdentifier(GTSpan(42, 53), "UserComment"),
              GTIdentifier(GTSpan(134, 147), "SystemComment"),
              GTIdentifier(GTSpan(209, 214), "False"),
              GTIdentifier(GTSpan(223, 228), "Float"),
              GTIdentifier(GTSpan(241, 247), "Number"),
              GTIdentifier(GTSpan(260, 266), "String"),
              GTIdentifier(GTSpan(292, 303), "NegativeInt"),
              GTIdentifier(GTSpan(309, 322), "NegativeFloat"),
              GTIdentifier(GTSpan(330, 340), "LargeFloat"),
              GTIdentifier(GTSpan(347, 357), "SmallFloat"),
            ],
            references: [
              GTIdentifier(GTSpan(62, 73), "CommentBase"),
              GTIdentifier(GTSpan(156, 167), "CommentBase"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/11-literals.type",
            source: "CommentBase: {\n  v: 2,\n  text: string,\n}\n\nUserComment: {\n  ...CommentBase,\n  type: \"user\",\n  userId: string,\n  published: boolean,\n}\n\nSystemComment: {\n  ...CommentBase,\n  type: \"system\",\n  published: true,\n}\n\nFalse: false\n\nFloat: 1.000_123\n\nNumber: 1_234_567\n\nString: \"Hello, \\\"world\\\"! \\\\\"\n\nNegativeInt: -1\n\nNegativeFloat: -1.0\n\nLargeFloat: 1e6\n\nSmallFloat: 3.5e-4\n",
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
                    Literal(GTLiteral(
                      span: GTSpan(7, 14),
                      doc: None,
                      attributes: [],
                      value: String("Sasha"),
                    )),
                    Literal(GTLiteral(
                      span: GTSpan(17, 24),
                      doc: None,
                      attributes: [],
                      value: String("world"),
                    )),
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
                    Literal(GTLiteral(
                      span: GTSpan(41, 48),
                      doc: None,
                      attributes: [],
                      value: String("Hello"),
                    )),
                    Primitive(GTPrimitive(
                      span: GTSpan(53, 59),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
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
                    Literal(GTLiteral(
                      span: GTSpan(102, 109),
                      doc: None,
                      attributes: [],
                      value: String("Hello"),
                    )),
                    Primitive(GTPrimitive(
                      span: GTSpan(141, 147),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "ObjectUnion"),
                span: GTSpan(149, 204),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(149, 160), "ObjectUnion"),
                descriptor: Union(GTUnion(
                  span: GTSpan(164, 204),
                  descriptors: [
                    Reference(GTReference(
                      span: GTSpan(166, 181),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(166, 181)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(166, 181), "ObjectUnionUser"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(186, 204),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(186, 204)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(186, 204), "ObjectUnionAccount"),
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "ObjectUnionUser"),
                span: GTSpan(206, 242),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(206, 221), "ObjectUnionUser"),
                descriptor: Object(GTObject(
                  span: GTSpan(223, 242),
                  name: Named(GTIdentifier(GTSpan(206, 221), "ObjectUnionUser")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(227, 239),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(227, 231), "kind"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(233, 239),
                        doc: None,
                        attributes: [],
                        value: String("user"),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "ObjectUnionAccount"),
                span: GTSpan(244, 286),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(244, 262), "ObjectUnionAccount"),
                descriptor: Object(GTObject(
                  span: GTSpan(264, 286),
                  name: Named(GTIdentifier(GTSpan(244, 262), "ObjectUnionAccount")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(268, 283),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(268, 272), "kind"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(274, 283),
                        doc: None,
                        attributes: [],
                        value: String("account"),
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
              GTIdentifier(GTSpan(26, 35), "Multiline"),
              GTIdentifier(GTSpan(61, 73), "WithComments"),
              GTIdentifier(GTSpan(149, 160), "ObjectUnion"),
              GTIdentifier(GTSpan(206, 221), "ObjectUnionUser"),
              GTIdentifier(GTSpan(244, 262), "ObjectUnionAccount"),
            ],
            references: [
              GTIdentifier(GTSpan(166, 181), "ObjectUnionUser"),
              GTIdentifier(GTSpan(186, 204), "ObjectUnionAccount"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/12-unions.type",
            source: "Hello: \"Sasha\" | \"world\"\n\nMultiline:\n  | \"Hello\"\n  | string\n\nWithComments:\n  // This is a comment\n  | \"Hello\"\n  // This is a comment too\n  | string\n\nObjectUnion:\n  | ObjectUnionUser\n  | ObjectUnionAccount\n\nObjectUnionUser: {\n  kind: \"user\",\n}\n\nObjectUnionAccount: {\n  kind: \"account\",\n}",
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
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(9, 14)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(9, 14), "Reply"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(17, 19),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(17, 19)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(17, 19), "DM"),
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Reply"),
                span: GTSpan(21, 76),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(21, 26), "Reply"),
                descriptor: Object(GTObject(
                  span: GTSpan(28, 76),
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
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(47, 54),
                        doc: None,
                        attributes: [],
                        value: String("reply"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(58, 73),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(58, 65), "message"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(67, 73),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "DM"),
                span: GTSpan(78, 127),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(78, 80), "DM"),
                descriptor: Object(GTObject(
                  span: GTSpan(82, 127),
                  name: Named(GTIdentifier(GTSpan(78, 80), "DM")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(86, 105),
                      doc: None,
                      attributes: [
                        GTAttribute(
                          span: GTSpan(86, 92),
                          name: GTAttributeName(
                            span: GTSpan(88, 91),
                            name: "tag",
                          ),
                          descriptor: None,
                        ),
                      ],
                      name: GTKey(GTSpan(95, 99), "type"),
                      descriptor: Literal(GTLiteral(
                        span: GTSpan(101, 105),
                        doc: None,
                        attributes: [],
                        value: String("dm"),
                      )),
                      required: true,
                    ),
                    GTProperty(
                      span: GTSpan(109, 124),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(109, 116), "message"),
                      descriptor: Primitive(GTPrimitive(
                        span: GTSpan(118, 124),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Assignment"),
                span: GTSpan(129, 163),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(129, 147),
                    name: GTAttributeName(
                      span: GTSpan(131, 136),
                      name: "hello",
                    ),
                    descriptor: Some(Assignment(GTAttributeAssignment(
                      span: GTSpan(137, 146),
                      value: Literal(GTLiteral(
                        span: GTSpan(139, 146),
                        doc: None,
                        attributes: [],
                        value: String("world"),
                      )),
                    ))),
                  ),
                ],
                name: GTIdentifier(GTSpan(148, 158), "Assignment"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(160, 163),
                  doc: None,
                  attributes: [],
                  value: Integer(123),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Arguments"),
                span: GTSpan(165, 207),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(165, 191),
                    name: GTAttributeName(
                      span: GTSpan(167, 172),
                      name: "hello",
                    ),
                    descriptor: Some(Arguments([
                      Literal(GTLiteral(
                        span: GTSpan(173, 180),
                        doc: None,
                        attributes: [],
                        value: String("cruel"),
                      )),
                      Literal(GTLiteral(
                        span: GTSpan(182, 189),
                        doc: None,
                        attributes: [],
                        value: String("world"),
                      )),
                    ])),
                  ),
                ],
                name: GTIdentifier(GTSpan(192, 201), "Arguments"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(203, 207),
                  doc: None,
                  attributes: [],
                  value: Boolean(true),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Properties"),
                span: GTSpan(209, 267),
                doc: None,
                attributes: [
                  GTAttribute(
                    span: GTSpan(209, 250),
                    name: GTAttributeName(
                      span: GTSpan(211, 216),
                      name: "hello",
                    ),
                    descriptor: Some(Properties([
                      GTAttributeProperty(
                        span: GTSpan(217, 232),
                        name: GTAttributeKey(
                          span: GTSpan(217, 222),
                          name: "which",
                        ),
                        value: Literal(GTLiteral(
                          span: GTSpan(225, 232),
                          doc: None,
                          attributes: [],
                          value: String("cruel"),
                        )),
                      ),
                      GTAttributeProperty(
                        span: GTSpan(234, 248),
                        name: GTAttributeKey(
                          span: GTSpan(234, 238),
                          name: "what",
                        ),
                        value: Literal(GTLiteral(
                          span: GTSpan(241, 248),
                          doc: None,
                          attributes: [],
                          value: String("world"),
                        )),
                      ),
                    ])),
                  ),
                ],
                name: GTIdentifier(GTSpan(251, 261), "Properties"),
                descriptor: Literal(GTLiteral(
                  span: GTSpan(263, 267),
                  doc: None,
                  attributes: [],
                  value: Boolean(true),
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Response"),
                span: GTSpan(269, 344),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(269, 277), "Response"),
                descriptor: Union(GTUnion(
                  span: GTSpan(279, 344),
                  descriptors: [
                    Reference(GTReference(
                      span: GTSpan(297, 312),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(297, 312)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(297, 312), "SuccessResponse"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(331, 344),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(331, 344)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(331, 344), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Response"),
                span: GTSpan(346, 431),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(346, 354), "Response"),
                descriptor: Union(GTUnion(
                  span: GTSpan(358, 431),
                  descriptors: [
                    Reference(GTReference(
                      span: GTSpan(380, 395),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(380, 395)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(380, 395), "SuccessResponse"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(418, 431),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(418, 431)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(418, 431), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Response"),
                span: GTSpan(433, 518),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(433, 441), "Response"),
                descriptor: Union(GTUnion(
                  span: GTSpan(445, 518),
                  descriptors: [
                    Reference(GTReference(
                      span: GTSpan(465, 480),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(465, 480)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(465, 480), "SuccessResponse"),
                    )),
                    Reference(GTReference(
                      span: GTSpan(503, 516),
                      doc: None,
                      attributes: [],
                      id: GTReferenceId(GTModuleId("module"), GTSpan(503, 516)),
                      definition_id: Unresolved,
                      identifier: GTIdentifier(GTSpan(503, 516), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
            ],
          ),
          resolve: GTModuleResolve(
            deps: [],
            exports: [
              GTIdentifier(GTSpan(0, 7), "Message"),
              GTIdentifier(GTSpan(21, 26), "Reply"),
              GTIdentifier(GTSpan(78, 80), "DM"),
              GTIdentifier(GTSpan(148, 158), "Assignment"),
              GTIdentifier(GTSpan(192, 201), "Arguments"),
              GTIdentifier(GTSpan(251, 261), "Properties"),
              GTIdentifier(GTSpan(269, 277), "Response"),
              GTIdentifier(GTSpan(346, 354), "Response"),
              GTIdentifier(GTSpan(433, 441), "Response"),
            ],
            references: [
              GTIdentifier(GTSpan(9, 14), "Reply"),
              GTIdentifier(GTSpan(17, 19), "DM"),
              GTIdentifier(GTSpan(297, 312), "SuccessResponse"),
              GTIdentifier(GTSpan(331, 344), "ErrorResponse"),
              GTIdentifier(GTSpan(380, 395), "SuccessResponse"),
              GTIdentifier(GTSpan(418, 431), "ErrorResponse"),
              GTIdentifier(GTSpan(465, 480), "SuccessResponse"),
              GTIdentifier(GTSpan(503, 516), "ErrorResponse"),
            ],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/13-attributes.type",
            source: "Message: Reply | DM\n\nReply: {\n  #[tag]\n  type: \"reply\",\n  message: string,\n}\n\nDM: {\n  #[tag]\n  type: \"dm\",\n  message: string,\n}\n\n#[hello = \"world\"]\nAssignment: 123\n\n#[hello(\"cruel\", \"world\")]\nArguments: true\n\n#[hello(which = \"cruel\", what = \"world\")]\nProperties: true\n\nResponse: #[name = Success] SuccessResponse | #[name = Error] ErrorResponse\n\nResponse:\n  | #[name = Success]\n  SuccessResponse\n  | #[name = Error]\n  ErrorResponse\n\nResponse:\n  #[name = Success]\n  SuccessResponse |\n  #[name = Error]\n  ErrorResponse |",
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(12, 18),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(36, 42),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  primitive: GTPrimitive(
                    span: GTSpan(8, 11),
                    kind: Int64,
                    doc: None,
                    attributes: [],
                  ),
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
                  primitive: GTPrimitive(
                    span: GTSpan(22, 28),
                    kind: String,
                    doc: None,
                    attributes: [],
                  ),
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
                  primitive: GTPrimitive(
                    span: GTSpan(38, 43),
                    kind: Float64,
                    doc: None,
                    attributes: [],
                  ),
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
                  primitive: GTPrimitive(
                    span: GTSpan(51, 58),
                    kind: Boolean,
                    doc: None,
                    attributes: [],
                  ),
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
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../examples/02-syntax/16-branded.type",
            source: "OrgId: @int\n\nUserId: @string\n\nConst: @float\n\nYes: @boolean",
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(6, 8),
                  kind: Int8,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int16"),
                span: GTSpan(9, 19),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(9, 14), "Int16"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(16, 19),
                  kind: Int16,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int32"),
                span: GTSpan(20, 30),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(20, 25), "Int32"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(27, 30),
                  kind: Int32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int64"),
                span: GTSpan(31, 41),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(31, 36), "Int64"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(38, 41),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Int128"),
                span: GTSpan(42, 54),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(42, 48), "Int128"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(50, 54),
                  kind: Int128,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntSize"),
                span: GTSpan(55, 69),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(55, 62), "IntSize"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(64, 69),
                  kind: IntSize,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU8"),
                span: GTSpan(70, 79),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(70, 75), "IntU8"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(77, 79),
                  kind: IntU8,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU16"),
                span: GTSpan(80, 91),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(80, 86), "IntU16"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(88, 91),
                  kind: IntU16,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU32"),
                span: GTSpan(92, 103),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(92, 98), "IntU32"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(100, 103),
                  kind: IntU32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU64"),
                span: GTSpan(104, 115),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(104, 110), "IntU64"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(112, 115),
                  kind: IntU64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntU128"),
                span: GTSpan(116, 129),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(116, 123), "IntU128"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(125, 129),
                  kind: IntU128,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "IntUSize"),
                span: GTSpan(130, 145),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(130, 138), "IntUSize"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(140, 145),
                  kind: IntUSize,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float32"),
                span: GTSpan(146, 158),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(146, 153), "Float32"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(155, 158),
                  kind: Float32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GTAlias(
                id: GTDefinitionId(GTModuleId("module"), "Float64"),
                span: GTSpan(159, 171),
                doc: None,
                attributes: [],
                name: GTIdentifier(GTSpan(159, 166), "Float64"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(168, 171),
                  kind: Float64,
                  doc: None,
                  attributes: [],
                )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(193, 199),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(224, 230),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(255, 261),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(286, 292),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(319, 325),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(354, 360),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(384, 390),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(416, 422),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(448, 454),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(480, 486),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(514, 520),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(550, 556),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(583, 589),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(616, 622),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(7, 13),
                  kind: Number,
                  doc: None,
                  attributes: [],
                )),
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
                  descriptor: Primitive(GTPrimitive(
                    span: GTSpan(34, 40),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
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
