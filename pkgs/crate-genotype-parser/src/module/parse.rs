use crate::prelude::internal::*;

/// Module parse result. It contains the module tree and resolve data.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtModuleParse {
    /// Module tree.
    pub module: GtModule,
    /// Module resolve. It contains module meta information used to build
    /// the dependency graph.
    pub resolve: GtModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[serde(serialize_with = "crate::miette_serde::serialize_named_source")]
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GtModule {
    pub fn parse<'a>(id: GtModuleId, source_code: NamedSource<String>) -> Result<GtModuleParse> {
        match parse_gt_code(source_code.inner()) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => match Self::parse_pairs(id.clone(), pair) {
                    Ok(result) => Ok(GtModuleParse {
                        resolve: result.resolve,
                        module: GtModule {
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
                    Err(GtModuleParseError::from_node_error(
                        source_code,
                        GtParseError::Internal(span, GtNode::Module),
                    )
                    .into())
                }
            },

            Err(error) => Err(GtModuleParseError::from_pest_error(source_code, error).into()),
        }
    }

    fn parse_pairs(
        module_id: GtModuleId,
        module_pair: Pair<'_, Rule>,
    ) -> Result<ModuleParseResult> {
        let mut doc: Option<GtDoc> = None;
        let mut imports = vec![];
        let mut aliases = vec![];
        let mut context = GtContext::new(module_id);

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
                    imports.push(GtImport::parse(pair, &mut context)?);
                }

                Rule::alias => {
                    aliases.push(GtAlias::parse(pair, &mut context)?);
                }

                Rule::EOI => {}

                rule => {
                    return Err(GtParseError::UnexpectedRule(
                        pair.as_span().into(),
                        GtNode::Module,
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
    doc: Option<GtDoc>,
    imports: Vec<GtImport>,
    aliases: Vec<GtAlias>,
    resolve: GtModuleResolve,
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;
    use miette::NamedSource;
    use std::fs;

    use super::*;

    #[test]
    fn test_alias() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/01-alias.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Age"),
                span: GtSpan(0, 8),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 3), "Age"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(5, 8),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "AnotherAge"),
                span: GtSpan(10, 25),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(10, 20), "AnotherAge"),
                descriptor: Reference(GtReference(
                  span: GtSpan(22, 25),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(22, 25)),
                  definition_id: Unresolved,
                  identifier: GtIdentifier(GtSpan(22, 25), "Age"),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "snake_case"),
                span: GtSpan(27, 42),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(27, 37), "snake_case"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(39, 42),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 3), "Age"),
              GtIdentifier(GtSpan(10, 20), "AnotherAge"),
              GtIdentifier(GtSpan(27, 37), "snake_case"),
            ],
            references: [
              GtIdentifier(GtSpan(22, 25), "Age"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/01-alias.type",
            source: "Age: int\n\nAnotherAge: Age\n\nsnake_case: int",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_primitives() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/02-primitives.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "String"),
                span: GtSpan(0, 14),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 6), "String"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(8, 14),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int"),
                span: GtSpan(16, 24),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(16, 19), "Int"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(21, 24),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float"),
                span: GtSpan(26, 38),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(26, 31), "Float"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(33, 38),
                  kind: Float64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Boolean"),
                span: GtSpan(40, 56),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(40, 47), "Boolean"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(49, 56),
                  kind: Boolean,
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 6), "String"),
              GtIdentifier(GtSpan(16, 19), "Int"),
              GtIdentifier(GtSpan(26, 31), "Float"),
              GtIdentifier(GtSpan(40, 47), "Boolean"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/02-primitives.type",
            source: "String: string\n\nInt: int\n\nFloat: float\n\nBoolean: boolean",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_objects() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/03-objects.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(0, 25),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(7, 25),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(11, 23),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(11, 15), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(27, 82),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(27, 32), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(34, 82),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(27, 32), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(38, 50),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(38, 42), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(44, 50),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(54, 62),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(54, 57), "age"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(59, 62),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(66, 79),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(66, 70), "flag"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(72, 79),
                        kind: Boolean,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Empty"),
                span: GtSpan(84, 93),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(84, 89), "Empty"),
                descriptor: Object(GtObject(
                  span: GtSpan(91, 93),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(84, 89), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Empty"),
                span: GtSpan(95, 106),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(95, 100), "Empty"),
                descriptor: Object(GtObject(
                  span: GtSpan(102, 106),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(95, 100), "Empty")),
                  extensions: [],
                  properties: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(108, 131),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(108, 113), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(115, 131),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(108, 113), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(117, 129),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(117, 121), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(123, 129),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(133, 166),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(133, 138), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(140, 166),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(133, 138), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(142, 154),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(142, 146), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(148, 154),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(156, 164),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(156, 159), "age"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(161, 164),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "PascalCase"),
                span: GtSpan(168, 202),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(168, 178), "PascalCase"),
                descriptor: Object(GtObject(
                  span: GtSpan(180, 202),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(168, 178), "PascalCase")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(184, 199),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(184, 194), "snake_case"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(196, 199),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "Hello"),
              GtIdentifier(GtSpan(27, 32), "Hello"),
              GtIdentifier(GtSpan(84, 89), "Empty"),
              GtIdentifier(GtSpan(95, 100), "Empty"),
              GtIdentifier(GtSpan(108, 113), "Hello"),
              GtIdentifier(GtSpan(133, 138), "Hello"),
              GtIdentifier(GtSpan(168, 178), "PascalCase"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/03-objects.type",
            source: "Hello: {\n  name: string\n}\n\nHello: {\n  name: string,\n  age: int,\n  flag: boolean,\n}\n\nEmpty: {}\n\nEmpty: {\n\n}\n\nHello: { name: string }\n\nHello: { name: string, age: int }\n\nPascalCase: {\n  snake_case: int,\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_comments() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/04-comments.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: Some(GtDoc(GtSpan(4, 38), "Module comment...\n...multiline")),
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(58, 110),
                doc: Some(GtDoc(GtSpan(62, 75), "Alias comment")),
                attributes: [],
                name: GtIdentifier(GtSpan(76, 81), "Hello"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(104, 110),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(112, 256),
                doc: Some(GtDoc(GtSpan(116, 149), "Multiline...\n...alias comment")),
                attributes: [],
                name: GtIdentifier(GtSpan(150, 155), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(157, 256),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(150, 155), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(161, 196),
                      doc: Some(GtDoc(GtSpan(165, 181), "Property comment")),
                      attributes: [],
                      name: GtKey(GtSpan(184, 188), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(190, 196),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(200, 253),
                      doc: Some(GtDoc(GtSpan(204, 242), "Multiline...\n...property comment")),
                      attributes: [],
                      name: GtKey(GtSpan(245, 248), "age"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(250, 253),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(258, 271),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(258, 263), "Hello"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(265, 271),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(76, 81), "Hello"),
              GtIdentifier(GtSpan(150, 155), "Hello"),
              GtIdentifier(GtSpan(258, 263), "Hello"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/04-comments.type",
            source: "//! Module comment...\n//! ...multiline\n\n// Basic comment\n\n/// Alias comment\nHello: /* Inline comment */ string\n\n/// Multiline...\n/// ...alias comment\nHello: {\n  /// Property comment\n  name: string,\n  /// Multiline...\n  /// ...property comment\n  age: int,\n}\n\nHello: string // Trailing comment",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_optional() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/05-optional.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(0, 38),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(7, 38),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(11, 23),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(11, 15), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(27, 36),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(27, 30), "age"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(33, 36),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "Hello"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/05-optional.type",
            source: "Hello: {\n  name: string,\n  age?: int\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_nested() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/06-nested.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(0, 61),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(7, 61),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 5), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(11, 59),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(11, 15), "name"),
                      descriptor: Object(GtObject(
                        span: GtSpan(17, 59),
                        doc: None,
                        attributes: [],
                        name: Alias(GtIdentifier(GtSpan(17, 59), "HelloName"), Property(GtIdentifier(GtSpan(0, 5), "Hello"), [
                          GtKey(GtSpan(11, 15), "name"),
                        ])),
                        extensions: [],
                        properties: [
                          GtProperty(
                            span: GtSpan(23, 36),
                            doc: None,
                            attributes: [],
                            name: GtKey(GtSpan(23, 28), "first"),
                            descriptor: Primitive(GtPrimitive(
                              span: GtSpan(30, 36),
                              kind: String,
                              doc: None,
                              attributes: [],
                            )),
                            required: true,
                          ),
                          GtProperty(
                            span: GtSpan(42, 54),
                            doc: None,
                            attributes: [],
                            name: GtKey(GtSpan(42, 46), "last"),
                            descriptor: Primitive(GtPrimitive(
                              span: GtSpan(48, 54),
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
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(63, 131),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(63, 68), "Hello"),
                descriptor: Object(GtObject(
                  span: GtSpan(70, 131),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(63, 68), "Hello")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(74, 129),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(74, 78), "name"),
                      descriptor: Alias(GtAlias(
                        id: GtDefinitionId(GtModuleId("module"), "Named"),
                        span: GtSpan(80, 129),
                        doc: None,
                        attributes: [],
                        name: GtIdentifier(GtSpan(80, 85), "Named"),
                        descriptor: Object(GtObject(
                          span: GtSpan(87, 129),
                          doc: None,
                          attributes: [],
                          name: Named(GtIdentifier(GtSpan(80, 85), "Named")),
                          extensions: [],
                          properties: [
                            GtProperty(
                              span: GtSpan(93, 106),
                              doc: None,
                              attributes: [],
                              name: GtKey(GtSpan(93, 98), "first"),
                              descriptor: Primitive(GtPrimitive(
                                span: GtSpan(100, 106),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
                              required: true,
                            ),
                            GtProperty(
                              span: GtSpan(112, 124),
                              doc: None,
                              attributes: [],
                              name: GtKey(GtSpan(112, 116), "last"),
                              descriptor: Primitive(GtPrimitive(
                                span: GtSpan(118, 124),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "Hello"),
              GtIdentifier(GtSpan(63, 68), "Hello"),
              GtIdentifier(GtSpan(80, 85), "Named"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/06-nested.type",
            source: "Hello: {\n  name: {\n    first: string,\n    last: string,\n  }\n}\n\nHello: {\n  name: Named: {\n    first: string,\n    last: string,\n  }\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_arrays() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/07-arrays.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Book"),
                span: GtSpan(0, 44),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 4), "Book"),
                descriptor: Object(GtObject(
                  span: GtSpan(6, 44),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 4), "Book")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(10, 23),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(10, 15), "title"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(17, 23),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(27, 41),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(27, 31), "tags"),
                      descriptor: Array(GtArray(
                        span: GtSpan(33, 41),
                        doc: None,
                        attributes: [],
                        descriptor: Primitive(GtPrimitive(
                          span: GtSpan(34, 40),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 4), "Book"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/07-arrays.type",
            source: "Book: {\n  title: string,\n  tags: [string],\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_tuples() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/08-tuples.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "User"),
                span: GtSpan(0, 69),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 4), "User"),
                descriptor: Object(GtObject(
                  span: GtSpan(6, 69),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 4), "User")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(10, 32),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(10, 14), "name"),
                      descriptor: Tuple(GtTuple(
                        span: GtSpan(16, 32),
                        doc: None,
                        attributes: [],
                        descriptors: [
                          Primitive(GtPrimitive(
                            span: GtSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GtPrimitive(
                            span: GtSpan(25, 31),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                        ],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(36, 66),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(36, 43), "address"),
                      descriptor: Tuple(GtTuple(
                        span: GtSpan(45, 66),
                        doc: None,
                        attributes: [],
                        descriptors: [
                          Primitive(GtPrimitive(
                            span: GtSpan(46, 49),
                            kind: Int64,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GtPrimitive(
                            span: GtSpan(51, 57),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GtPrimitive(
                            span: GtSpan(59, 65),
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
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Address"),
                span: GtSpan(71, 101),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(71, 78), "Address"),
                descriptor: Tuple(GtTuple(
                  span: GtSpan(80, 101),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Primitive(GtPrimitive(
                      span: GtSpan(81, 84),
                      kind: Int64,
                      doc: None,
                      attributes: [],
                    )),
                    Primitive(GtPrimitive(
                      span: GtSpan(86, 92),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                    Primitive(GtPrimitive(
                      span: GtSpan(94, 100),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Empty"),
                span: GtSpan(103, 112),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(103, 108), "Empty"),
                descriptor: Tuple(GtTuple(
                  span: GtSpan(110, 112),
                  doc: None,
                  attributes: [],
                  descriptors: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 4), "User"),
              GtIdentifier(GtSpan(71, 78), "Address"),
              GtIdentifier(GtSpan(103, 108), "Empty"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/08-tuples.type",
            source: "User: {\n  name: (string, string),\n  address: (int, string, string),\n}\n\nAddress: (int, string, string)\n\nEmpty: ()",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_modules() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/09-modules.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [
              GtImport(
                span: GtSpan(0, 12),
                path: GtPath(
                  span: GtSpan(4, 10),
                  id: GtPathModuleId(
                    span: GtSpan(4, 10),
                    module_id: GtModuleId("module"),
                  ),
                  path: "author",
                ),
                reference: Glob(GtSpan(11, 12)),
              ),
              GtImport(
                span: GtSpan(13, 64),
                path: GtPath(
                  span: GtSpan(17, 29),
                  id: GtPathModuleId(
                    span: GtSpan(17, 29),
                    module_id: GtModuleId("module"),
                  ),
                  path: "../../author",
                ),
                reference: Names(GtSpan(30, 64), [
                  Name(GtSpan(31, 37), GtIdentifier(GtSpan(31, 37), "Author")),
                  Name(GtSpan(39, 44), GtIdentifier(GtSpan(39, 44), "Genre")),
                  Alias(GtSpan(46, 63), GtIdentifier(GtSpan(46, 55), "Something"), GtIdentifier(GtSpan(59, 63), "Else")),
                ]),
              ),
              GtImport(
                span: GtSpan(65, 82),
                path: GtPath(
                  span: GtSpan(69, 75),
                  id: GtPathModuleId(
                    span: GtSpan(69, 75),
                    module_id: GtModuleId("module"),
                  ),
                  path: "author",
                ),
                reference: Name(GtSpan(76, 82), GtIdentifier(GtSpan(76, 82), "Author")),
              ),
            ],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Book"),
                span: GtSpan(84, 157),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(84, 88), "Book"),
                descriptor: Object(GtObject(
                  span: GtSpan(90, 157),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(84, 88), "Book")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(94, 107),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(94, 99), "title"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(101, 107),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(111, 138),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(111, 117), "author"),
                      descriptor: InlineImport(GtInlineImport(
                        span: GtSpan(119, 138),
                        doc: None,
                        attributes: [],
                        name: GtIdentifier(GtSpan(132, 138), "Author"),
                        path: GtPath(
                          span: GtSpan(119, 131),
                          id: GtPathModuleId(
                            span: GtSpan(119, 131),
                            module_id: GtModuleId("module"),
                          ),
                          path: "../../author",
                        ),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(142, 154),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(142, 147), "genre"),
                      descriptor: Reference(GtReference(
                        span: GtSpan(149, 154),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(149, 154)),
                        definition_id: Unresolved,
                        identifier: GtIdentifier(GtSpan(149, 154), "Genre"),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Author"),
                span: GtSpan(159, 186),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(159, 165), "Author"),
                descriptor: InlineImport(GtInlineImport(
                  span: GtSpan(167, 186),
                  doc: None,
                  attributes: [],
                  name: GtIdentifier(GtSpan(180, 186), "Author"),
                  path: GtPath(
                    span: GtSpan(167, 179),
                    id: GtPathModuleId(
                      span: GtSpan(167, 179),
                      module_id: GtModuleId("module"),
                    ),
                    path: "../../author",
                  ),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Authors"),
                span: GtSpan(188, 218),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(188, 195), "Authors"),
                descriptor: Array(GtArray(
                  span: GtSpan(197, 218),
                  doc: None,
                  attributes: [],
                  descriptor: InlineImport(GtInlineImport(
                    span: GtSpan(198, 217),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(211, 217), "Author"),
                    path: GtPath(
                      span: GtSpan(198, 210),
                      id: GtPathModuleId(
                        span: GtSpan(198, 210),
                        module_id: GtModuleId("module"),
                      ),
                      path: "../../author",
                    ),
                  )),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [
              GtPath(
                span: GtSpan(4, 10),
                id: GtPathModuleId(
                  span: GtSpan(4, 10),
                  module_id: GtModuleId("module"),
                ),
                path: "author",
              ),
              GtPath(
                span: GtSpan(17, 29),
                id: GtPathModuleId(
                  span: GtSpan(17, 29),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
              GtPath(
                span: GtSpan(69, 75),
                id: GtPathModuleId(
                  span: GtSpan(69, 75),
                  module_id: GtModuleId("module"),
                ),
                path: "author",
              ),
              GtPath(
                span: GtSpan(119, 131),
                id: GtPathModuleId(
                  span: GtSpan(119, 131),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
              GtPath(
                span: GtSpan(167, 179),
                id: GtPathModuleId(
                  span: GtSpan(167, 179),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
              GtPath(
                span: GtSpan(198, 210),
                id: GtPathModuleId(
                  span: GtSpan(198, 210),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
            ],
            exports: [
              GtIdentifier(GtSpan(84, 88), "Book"),
              GtIdentifier(GtSpan(159, 165), "Author"),
              GtIdentifier(GtSpan(188, 195), "Authors"),
            ],
            references: [
              GtIdentifier(GtSpan(149, 154), "Genre"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/09-modules.type",
            source: "use author/*\nuse ../../author/{Author, Genre, Something as Else}\nuse author/Author\n\nBook: {\n  title: string,\n  author: ../../author/Author,\n  genre: Genre,\n}\n\nAuthor: ../../author/Author\n\nAuthors: [../../author/Author]",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_extensions() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/10-extensions.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Base"),
                span: GtSpan(0, 37),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 4), "Base"),
                descriptor: Object(GtObject(
                  span: GtSpan(6, 37),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 4), "Base")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(10, 22),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(10, 14), "name"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(16, 22),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(26, 34),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(26, 29), "age"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(31, 34),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Processor"),
                span: GtSpan(39, 78),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(39, 48), "Processor"),
                descriptor: Object(GtObject(
                  span: GtSpan(50, 78),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(39, 48), "Processor")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(54, 61),
                      reference: GtReference(
                        span: GtSpan(57, 61),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(57, 61)),
                        definition_id: Unresolved,
                        identifier: GtIdentifier(GtSpan(57, 61), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GtProperty(
                      span: GtSpan(65, 75),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(65, 70), "cores"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(72, 75),
                        kind: Int64,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "User"),
                span: GtSpan(80, 117),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(80, 84), "User"),
                descriptor: Object(GtObject(
                  span: GtSpan(86, 117),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(80, 84), "User")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(90, 97),
                      reference: GtReference(
                        span: GtSpan(93, 97),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(93, 97)),
                        definition_id: Unresolved,
                        identifier: GtIdentifier(GtSpan(93, 97), "Base"),
                      ),
                    ),
                  ],
                  properties: [
                    GtProperty(
                      span: GtSpan(101, 114),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(101, 106), "email"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(108, 114),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 4), "Base"),
              GtIdentifier(GtSpan(39, 48), "Processor"),
              GtIdentifier(GtSpan(80, 84), "User"),
            ],
            references: [
              GtIdentifier(GtSpan(57, 61), "Base"),
              GtIdentifier(GtSpan(93, 97), "Base"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/10-extensions.type",
            source: "Base: {\n  name: string,\n  age: int,\n}\n\nProcessor: {\n  ...Base,\n  cores: int,\n}\n\nUser: {\n  ...Base,\n  email: string,\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_literals() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/11-literals.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "CommentBase"),
                span: GtSpan(0, 40),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 11), "CommentBase"),
                descriptor: Object(GtObject(
                  span: GtSpan(13, 40),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 11), "CommentBase")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(17, 21),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(17, 18), "v"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(20, 21),
                        doc: None,
                        attributes: [],
                        value: Integer(2),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(25, 37),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(25, 29), "text"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(31, 37),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "UserComment"),
                span: GtSpan(42, 132),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(42, 53), "UserComment"),
                descriptor: Object(GtObject(
                  span: GtSpan(55, 132),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(42, 53), "UserComment")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(59, 73),
                      reference: GtReference(
                        span: GtSpan(62, 73),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(62, 73)),
                        definition_id: Unresolved,
                        identifier: GtIdentifier(GtSpan(62, 73), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GtProperty(
                      span: GtSpan(77, 89),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(77, 81), "type"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(83, 89),
                        doc: None,
                        attributes: [],
                        value: String("user"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(93, 107),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(93, 99), "userId"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(101, 107),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(111, 129),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(111, 120), "published"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(122, 129),
                        kind: Boolean,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "SystemComment"),
                span: GtSpan(134, 207),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(134, 147), "SystemComment"),
                descriptor: Object(GtObject(
                  span: GtSpan(149, 207),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(134, 147), "SystemComment")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(153, 167),
                      reference: GtReference(
                        span: GtSpan(156, 167),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(156, 167)),
                        definition_id: Unresolved,
                        identifier: GtIdentifier(GtSpan(156, 167), "CommentBase"),
                      ),
                    ),
                  ],
                  properties: [
                    GtProperty(
                      span: GtSpan(171, 185),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(171, 175), "type"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(177, 185),
                        doc: None,
                        attributes: [],
                        value: String("system"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(189, 204),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(189, 198), "published"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(200, 204),
                        doc: None,
                        attributes: [],
                        value: Boolean(true),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "False"),
                span: GtSpan(209, 221),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(209, 214), "False"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(216, 221),
                  doc: None,
                  attributes: [],
                  value: Boolean(false),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float"),
                span: GtSpan(223, 239),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(223, 228), "Float"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(230, 239),
                  doc: None,
                  attributes: [],
                  value: Float(1.000123),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Number"),
                span: GtSpan(241, 258),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(241, 247), "Number"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(249, 258),
                  doc: None,
                  attributes: [],
                  value: Integer(1234567),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "String"),
                span: GtSpan(260, 290),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(260, 266), "String"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(268, 290),
                  doc: None,
                  attributes: [],
                  value: String("Hello, \\\"world\\\"! \\\\"),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "NegativeInt"),
                span: GtSpan(292, 307),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(292, 303), "NegativeInt"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(305, 307),
                  doc: None,
                  attributes: [],
                  value: Integer(-1),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "NegativeFloat"),
                span: GtSpan(309, 328),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(309, 322), "NegativeFloat"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(324, 328),
                  doc: None,
                  attributes: [],
                  value: Float(-1.0),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "LargeFloat"),
                span: GtSpan(330, 345),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(330, 340), "LargeFloat"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(342, 345),
                  doc: None,
                  attributes: [],
                  value: Float(1000000.0),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "SmallFloat"),
                span: GtSpan(347, 365),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(347, 357), "SmallFloat"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(359, 365),
                  doc: None,
                  attributes: [],
                  value: Float(0.00035),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 11), "CommentBase"),
              GtIdentifier(GtSpan(42, 53), "UserComment"),
              GtIdentifier(GtSpan(134, 147), "SystemComment"),
              GtIdentifier(GtSpan(209, 214), "False"),
              GtIdentifier(GtSpan(223, 228), "Float"),
              GtIdentifier(GtSpan(241, 247), "Number"),
              GtIdentifier(GtSpan(260, 266), "String"),
              GtIdentifier(GtSpan(292, 303), "NegativeInt"),
              GtIdentifier(GtSpan(309, 322), "NegativeFloat"),
              GtIdentifier(GtSpan(330, 340), "LargeFloat"),
              GtIdentifier(GtSpan(347, 357), "SmallFloat"),
            ],
            references: [
              GtIdentifier(GtSpan(62, 73), "CommentBase"),
              GtIdentifier(GtSpan(156, 167), "CommentBase"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/11-literals.type",
            source: "CommentBase: {\n  v: 2,\n  text: string,\n}\n\nUserComment: {\n  ...CommentBase,\n  type: \"user\",\n  userId: string,\n  published: boolean,\n}\n\nSystemComment: {\n  ...CommentBase,\n  type: \"system\",\n  published: true,\n}\n\nFalse: false\n\nFloat: 1.000_123\n\nNumber: 1_234_567\n\nString: \"Hello, \\\"world\\\"! \\\\\"\n\nNegativeInt: -1\n\nNegativeFloat: -1.0\n\nLargeFloat: 1e6\n\nSmallFloat: 3.5e-4\n",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_unions() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/12-unions.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(0, 24),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "Hello"),
                descriptor: Union(GtUnion(
                  span: GtSpan(7, 24),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Literal(GtLiteral(
                      span: GtSpan(7, 14),
                      doc: None,
                      attributes: [],
                      value: String("Sasha"),
                    )),
                    Literal(GtLiteral(
                      span: GtSpan(17, 24),
                      doc: None,
                      attributes: [],
                      value: String("world"),
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Multiline"),
                span: GtSpan(26, 59),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(26, 35), "Multiline"),
                descriptor: Union(GtUnion(
                  span: GtSpan(39, 59),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Literal(GtLiteral(
                      span: GtSpan(41, 48),
                      doc: None,
                      attributes: [],
                      value: String("Hello"),
                    )),
                    Primitive(GtPrimitive(
                      span: GtSpan(53, 59),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "WithComments"),
                span: GtSpan(61, 147),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(61, 73), "WithComments"),
                descriptor: Union(GtUnion(
                  span: GtSpan(100, 147),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Literal(GtLiteral(
                      span: GtSpan(102, 109),
                      doc: None,
                      attributes: [],
                      value: String("Hello"),
                    )),
                    Primitive(GtPrimitive(
                      span: GtSpan(141, 147),
                      kind: String,
                      doc: None,
                      attributes: [],
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ObjectUnion"),
                span: GtSpan(149, 204),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(149, 160), "ObjectUnion"),
                descriptor: Union(GtUnion(
                  span: GtSpan(164, 204),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(166, 181),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(166, 181)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(166, 181), "ObjectUnionUser"),
                    )),
                    Reference(GtReference(
                      span: GtSpan(186, 204),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(186, 204)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(186, 204), "ObjectUnionAccount"),
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ObjectUnionUser"),
                span: GtSpan(206, 242),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(206, 221), "ObjectUnionUser"),
                descriptor: Object(GtObject(
                  span: GtSpan(223, 242),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(206, 221), "ObjectUnionUser")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(227, 239),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(227, 231), "kind"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(233, 239),
                        doc: None,
                        attributes: [],
                        value: String("user"),
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ObjectUnionAccount"),
                span: GtSpan(244, 286),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(244, 262), "ObjectUnionAccount"),
                descriptor: Object(GtObject(
                  span: GtSpan(264, 286),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(244, 262), "ObjectUnionAccount")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(268, 283),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(268, 272), "kind"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(274, 283),
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
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "Hello"),
              GtIdentifier(GtSpan(26, 35), "Multiline"),
              GtIdentifier(GtSpan(61, 73), "WithComments"),
              GtIdentifier(GtSpan(149, 160), "ObjectUnion"),
              GtIdentifier(GtSpan(206, 221), "ObjectUnionUser"),
              GtIdentifier(GtSpan(244, 262), "ObjectUnionAccount"),
            ],
            references: [
              GtIdentifier(GtSpan(166, 181), "ObjectUnionUser"),
              GtIdentifier(GtSpan(186, 204), "ObjectUnionAccount"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/12-unions.type",
            source: "Hello: \"Sasha\" | \"world\"\n\nMultiline:\n  | \"Hello\"\n  | string\n\nWithComments:\n  // This is a comment\n  | \"Hello\"\n  // This is a comment too\n  | string\n\nObjectUnion:\n  | ObjectUnionUser\n  | ObjectUnionAccount\n\nObjectUnionUser: {\n  kind: \"user\",\n}\n\nObjectUnionAccount: {\n  kind: \"account\",\n}",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_attributes() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/13-attributes.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Message"),
                span: GtSpan(0, 19),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 7), "Message"),
                descriptor: Union(GtUnion(
                  span: GtSpan(9, 19),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(9, 14),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(9, 14)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(9, 14), "Reply"),
                    )),
                    Reference(GtReference(
                      span: GtSpan(17, 19),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(17, 19)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(17, 19), "DM"),
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Reply"),
                span: GtSpan(21, 76),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(21, 26), "Reply"),
                descriptor: Object(GtObject(
                  span: GtSpan(28, 76),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(21, 26), "Reply")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(32, 54),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(32, 38),
                          name: GtAttributeName(
                            span: GtSpan(34, 37),
                            value: "tag",
                          ),
                          descriptor: None,
                        ),
                      ],
                      name: GtKey(GtSpan(41, 45), "type"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(47, 54),
                        doc: None,
                        attributes: [],
                        value: String("reply"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(58, 73),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(58, 65), "message"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(67, 73),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "DM"),
                span: GtSpan(78, 127),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(78, 80), "DM"),
                descriptor: Object(GtObject(
                  span: GtSpan(82, 127),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(78, 80), "DM")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(86, 105),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(86, 92),
                          name: GtAttributeName(
                            span: GtSpan(88, 91),
                            value: "tag",
                          ),
                          descriptor: None,
                        ),
                      ],
                      name: GtKey(GtSpan(95, 99), "type"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(101, 105),
                        doc: None,
                        attributes: [],
                        value: String("dm"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(109, 124),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(109, 116), "message"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(118, 124),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Assignment"),
                span: GtSpan(129, 163),
                doc: None,
                attributes: [
                  GtAttribute(
                    span: GtSpan(129, 147),
                    name: GtAttributeName(
                      span: GtSpan(131, 136),
                      value: "hello",
                    ),
                    descriptor: Some(Assignment(GtAttributeAssignment(
                      span: GtSpan(137, 146),
                      value: Literal(GtLiteral(
                        span: GtSpan(139, 146),
                        doc: None,
                        attributes: [],
                        value: String("world"),
                      )),
                    ))),
                  ),
                ],
                name: GtIdentifier(GtSpan(148, 158), "Assignment"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(160, 163),
                  doc: None,
                  attributes: [],
                  value: Integer(123),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Arguments"),
                span: GtSpan(165, 207),
                doc: None,
                attributes: [
                  GtAttribute(
                    span: GtSpan(165, 191),
                    name: GtAttributeName(
                      span: GtSpan(167, 172),
                      value: "hello",
                    ),
                    descriptor: Some(Arguments([
                      Literal(GtLiteral(
                        span: GtSpan(173, 180),
                        doc: None,
                        attributes: [],
                        value: String("cruel"),
                      )),
                      Literal(GtLiteral(
                        span: GtSpan(182, 189),
                        doc: None,
                        attributes: [],
                        value: String("world"),
                      )),
                    ])),
                  ),
                ],
                name: GtIdentifier(GtSpan(192, 201), "Arguments"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(203, 207),
                  doc: None,
                  attributes: [],
                  value: Boolean(true),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Properties"),
                span: GtSpan(209, 267),
                doc: None,
                attributes: [
                  GtAttribute(
                    span: GtSpan(209, 250),
                    name: GtAttributeName(
                      span: GtSpan(211, 216),
                      value: "hello",
                    ),
                    descriptor: Some(Properties([
                      GtAttributeProperty(
                        span: GtSpan(217, 232),
                        name: GtAttributeKey(
                          span: GtSpan(217, 222),
                          value: "which",
                        ),
                        value: Literal(GtLiteral(
                          span: GtSpan(225, 232),
                          doc: None,
                          attributes: [],
                          value: String("cruel"),
                        )),
                      ),
                      GtAttributeProperty(
                        span: GtSpan(234, 248),
                        name: GtAttributeKey(
                          span: GtSpan(234, 238),
                          value: "what",
                        ),
                        value: Literal(GtLiteral(
                          span: GtSpan(241, 248),
                          doc: None,
                          attributes: [],
                          value: String("world"),
                        )),
                      ),
                    ])),
                  ),
                ],
                name: GtIdentifier(GtSpan(251, 261), "Properties"),
                descriptor: Literal(GtLiteral(
                  span: GtSpan(263, 267),
                  doc: None,
                  attributes: [],
                  value: Boolean(true),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Response"),
                span: GtSpan(269, 344),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(269, 277), "Response"),
                descriptor: Union(GtUnion(
                  span: GtSpan(279, 344),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(297, 312),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(279, 296),
                          name: GtAttributeName(
                            span: GtSpan(281, 285),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(286, 295),
                            value: Identifier(GtIdentifier(GtSpan(288, 295), "Success")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(297, 312)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(297, 312), "SuccessResponse"),
                    )),
                    Reference(GtReference(
                      span: GtSpan(331, 344),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(315, 330),
                          name: GtAttributeName(
                            span: GtSpan(317, 321),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(322, 329),
                            value: Identifier(GtIdentifier(GtSpan(324, 329), "Error")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(331, 344)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(331, 344), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Response"),
                span: GtSpan(346, 431),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(346, 354), "Response"),
                descriptor: Union(GtUnion(
                  span: GtSpan(358, 431),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(380, 395),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(360, 377),
                          name: GtAttributeName(
                            span: GtSpan(362, 366),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(367, 376),
                            value: Identifier(GtIdentifier(GtSpan(369, 376), "Success")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(380, 395)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(380, 395), "SuccessResponse"),
                    )),
                    Reference(GtReference(
                      span: GtSpan(418, 431),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(400, 415),
                          name: GtAttributeName(
                            span: GtSpan(402, 406),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(407, 414),
                            value: Identifier(GtIdentifier(GtSpan(409, 414), "Error")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(418, 431)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(418, 431), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Response"),
                span: GtSpan(433, 518),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(433, 441), "Response"),
                descriptor: Union(GtUnion(
                  span: GtSpan(445, 518),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(465, 480),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(445, 462),
                          name: GtAttributeName(
                            span: GtSpan(447, 451),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(452, 461),
                            value: Identifier(GtIdentifier(GtSpan(454, 461), "Success")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(465, 480)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(465, 480), "SuccessResponse"),
                    )),
                    Reference(GtReference(
                      span: GtSpan(503, 516),
                      doc: None,
                      attributes: [
                        GtAttribute(
                          span: GtSpan(485, 500),
                          name: GtAttributeName(
                            span: GtSpan(487, 491),
                            value: "name",
                          ),
                          descriptor: Some(Assignment(GtAttributeAssignment(
                            span: GtSpan(492, 499),
                            value: Identifier(GtIdentifier(GtSpan(494, 499), "Error")),
                          ))),
                        ),
                      ],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(503, 516)),
                      definition_id: Unresolved,
                      identifier: GtIdentifier(GtSpan(503, 516), "ErrorResponse"),
                    )),
                  ],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 7), "Message"),
              GtIdentifier(GtSpan(21, 26), "Reply"),
              GtIdentifier(GtSpan(78, 80), "DM"),
              GtIdentifier(GtSpan(148, 158), "Assignment"),
              GtIdentifier(GtSpan(192, 201), "Arguments"),
              GtIdentifier(GtSpan(251, 261), "Properties"),
              GtIdentifier(GtSpan(269, 277), "Response"),
              GtIdentifier(GtSpan(346, 354), "Response"),
              GtIdentifier(GtSpan(433, 441), "Response"),
            ],
            references: [
              GtIdentifier(GtSpan(9, 14), "Reply"),
              GtIdentifier(GtSpan(17, 19), "DM"),
              GtIdentifier(GtSpan(297, 312), "SuccessResponse"),
              GtIdentifier(GtSpan(331, 344), "ErrorResponse"),
              GtIdentifier(GtSpan(380, 395), "SuccessResponse"),
              GtIdentifier(GtSpan(418, 431), "ErrorResponse"),
              GtIdentifier(GtSpan(465, 480), "SuccessResponse"),
              GtIdentifier(GtSpan(503, 516), "ErrorResponse"),
            ],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/13-attributes.type",
            source: "Message: Reply | DM\n\nReply: {\n  #[tag]\n  type: \"reply\",\n  message: string,\n}\n\nDM: {\n  #[tag]\n  type: \"dm\",\n  message: string,\n}\n\n#[hello = \"world\"]\nAssignment: 123\n\n#[hello(\"cruel\", \"world\")]\nArguments: true\n\n#[hello(which = \"cruel\", what = \"world\")]\nProperties: true\n\nResponse: #[name = Success] SuccessResponse | #[name = Error] ErrorResponse\n\nResponse:\n  | #[name = Success]\n  SuccessResponse\n  | #[name = Error]\n  ErrorResponse\n\nResponse:\n  #[name = Success]\n  SuccessResponse |\n  #[name = Error]\n  ErrorResponse |",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_records() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/14-records.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Dict"),
                span: GtSpan(0, 20),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 4), "Dict"),
                descriptor: Record(GtRecord(
                  span: GtSpan(6, 20),
                  doc: None,
                  attributes: [],
                  key: String(GtSpan(8, 10)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(12, 18),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Map"),
                span: GtSpan(22, 44),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(22, 25), "Map"),
                descriptor: Record(GtRecord(
                  span: GtSpan(27, 44),
                  doc: None,
                  attributes: [],
                  key: Int64(GtSpan(29, 34)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(36, 42),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 4), "Dict"),
              GtIdentifier(GtSpan(22, 25), "Map"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/14-records.type",
            source: "Dict: { []: string }\n\nMap: { [int]: string }",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_any() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/15-any.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Anything"),
                span: GtSpan(0, 13),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 8), "Anything"),
                descriptor: Any(GtAny(
                  span: GtSpan(10, 13),
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 8), "Anything"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/15-any.type",
            source: "Anything: any",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_branded() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/16-branded.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "OrgId"),
                span: GtSpan(0, 11),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "OrgId"),
                descriptor: Branded(GtBranded(
                  span: GtSpan(7, 11),
                  doc: None,
                  attributes: [],
                  id: GtDefinitionId(GtModuleId("module"), "OrgId"),
                  name: GtIdentifier(GtSpan(0, 5), "OrgId"),
                  primitive: GtPrimitive(
                    span: GtSpan(8, 11),
                    kind: Int64,
                    doc: None,
                    attributes: [],
                  ),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "UserId"),
                span: GtSpan(13, 28),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(13, 19), "UserId"),
                descriptor: Branded(GtBranded(
                  span: GtSpan(21, 28),
                  doc: None,
                  attributes: [],
                  id: GtDefinitionId(GtModuleId("module"), "UserId"),
                  name: GtIdentifier(GtSpan(13, 19), "UserId"),
                  primitive: GtPrimitive(
                    span: GtSpan(22, 28),
                    kind: String,
                    doc: None,
                    attributes: [],
                  ),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Const"),
                span: GtSpan(30, 43),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(30, 35), "Const"),
                descriptor: Branded(GtBranded(
                  span: GtSpan(37, 43),
                  doc: None,
                  attributes: [],
                  id: GtDefinitionId(GtModuleId("module"), "Const"),
                  name: GtIdentifier(GtSpan(30, 35), "Const"),
                  primitive: GtPrimitive(
                    span: GtSpan(38, 43),
                    kind: Float64,
                    doc: None,
                    attributes: [],
                  ),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Yes"),
                span: GtSpan(45, 58),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(45, 48), "Yes"),
                descriptor: Branded(GtBranded(
                  span: GtSpan(50, 58),
                  doc: None,
                  attributes: [],
                  id: GtDefinitionId(GtModuleId("module"), "Yes"),
                  name: GtIdentifier(GtSpan(45, 48), "Yes"),
                  primitive: GtPrimitive(
                    span: GtSpan(51, 58),
                    kind: Boolean,
                    doc: None,
                    attributes: [],
                  ),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "OrgId"),
              GtIdentifier(GtSpan(13, 19), "UserId"),
              GtIdentifier(GtSpan(30, 35), "Const"),
              GtIdentifier(GtSpan(45, 48), "Yes"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/16-branded.type",
            source: "OrgId: @int\n\nUserId: @string\n\nConst: @float\n\nYes: @boolean",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_number_sizes() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/17-number_sizes.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int8"),
                span: GtSpan(0, 8),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 4), "Int8"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(6, 8),
                  kind: Int8,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int16"),
                span: GtSpan(9, 19),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(9, 14), "Int16"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(16, 19),
                  kind: Int16,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int32"),
                span: GtSpan(20, 30),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(20, 25), "Int32"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(27, 30),
                  kind: Int32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int64"),
                span: GtSpan(31, 41),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(31, 36), "Int64"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(38, 41),
                  kind: Int64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int128"),
                span: GtSpan(42, 54),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(42, 48), "Int128"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(50, 54),
                  kind: Int128,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntSize"),
                span: GtSpan(55, 69),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(55, 62), "IntSize"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(64, 69),
                  kind: IntSize,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU8"),
                span: GtSpan(70, 79),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(70, 75), "IntU8"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(77, 79),
                  kind: IntU8,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU16"),
                span: GtSpan(80, 91),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(80, 86), "IntU16"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(88, 91),
                  kind: IntU16,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU32"),
                span: GtSpan(92, 103),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(92, 98), "IntU32"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(100, 103),
                  kind: IntU32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU64"),
                span: GtSpan(104, 115),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(104, 110), "IntU64"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(112, 115),
                  kind: IntU64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU128"),
                span: GtSpan(116, 129),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(116, 123), "IntU128"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(125, 129),
                  kind: IntU128,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntUSize"),
                span: GtSpan(130, 145),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(130, 138), "IntUSize"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(140, 145),
                  kind: IntUSize,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float32"),
                span: GtSpan(146, 158),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(146, 153), "Float32"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(155, 158),
                  kind: Float32,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float64"),
                span: GtSpan(159, 171),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(159, 166), "Float64"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(168, 171),
                  kind: Float64,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int8Record"),
                span: GtSpan(173, 201),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(173, 183), "Int8Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(185, 201),
                  doc: None,
                  attributes: [],
                  key: Int8(GtSpan(187, 191)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(193, 199),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int16Record"),
                span: GtSpan(202, 232),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(202, 213), "Int16Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(215, 232),
                  doc: None,
                  attributes: [],
                  key: Int16(GtSpan(217, 222)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(224, 230),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int32Record"),
                span: GtSpan(233, 263),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(233, 244), "Int32Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(246, 263),
                  doc: None,
                  attributes: [],
                  key: Int32(GtSpan(248, 253)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(255, 261),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int64Record"),
                span: GtSpan(264, 294),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(264, 275), "Int64Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(277, 294),
                  doc: None,
                  attributes: [],
                  key: Int64(GtSpan(279, 284)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(286, 292),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Int128Record"),
                span: GtSpan(295, 327),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(295, 307), "Int128Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(309, 327),
                  doc: None,
                  attributes: [],
                  key: Int128(GtSpan(311, 317)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(319, 325),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntSizeRecord"),
                span: GtSpan(328, 362),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(328, 341), "IntSizeRecord"),
                descriptor: Record(GtRecord(
                  span: GtSpan(343, 362),
                  doc: None,
                  attributes: [],
                  key: IntSize(GtSpan(345, 352)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(354, 360),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU8Record"),
                span: GtSpan(363, 392),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(363, 374), "IntU8Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(376, 392),
                  doc: None,
                  attributes: [],
                  key: IntU8(GtSpan(378, 382)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(384, 390),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU16Record"),
                span: GtSpan(393, 424),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(393, 405), "IntU16Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(407, 424),
                  doc: None,
                  attributes: [],
                  key: IntU16(GtSpan(409, 414)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(416, 422),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU32Record"),
                span: GtSpan(425, 456),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(425, 437), "IntU32Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(439, 456),
                  doc: None,
                  attributes: [],
                  key: IntU32(GtSpan(441, 446)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(448, 454),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU64Record"),
                span: GtSpan(457, 488),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(457, 469), "IntU64Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(471, 488),
                  doc: None,
                  attributes: [],
                  key: IntU64(GtSpan(473, 478)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(480, 486),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntU128Record"),
                span: GtSpan(489, 522),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(489, 502), "IntU128Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(504, 522),
                  doc: None,
                  attributes: [],
                  key: IntU128(GtSpan(506, 512)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(514, 520),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "IntUSizeRecord"),
                span: GtSpan(523, 558),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(523, 537), "IntUSizeRecord"),
                descriptor: Record(GtRecord(
                  span: GtSpan(539, 558),
                  doc: None,
                  attributes: [],
                  key: IntUSize(GtSpan(541, 548)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(550, 556),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float32Record"),
                span: GtSpan(559, 591),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(559, 572), "Float32Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(574, 591),
                  doc: None,
                  attributes: [],
                  key: Float32(GtSpan(576, 581)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(583, 589),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Float64Record"),
                span: GtSpan(592, 624),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(592, 605), "Float64Record"),
                descriptor: Record(GtRecord(
                  span: GtSpan(607, 624),
                  doc: None,
                  attributes: [],
                  key: Float64(GtSpan(609, 614)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(616, 622),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 4), "Int8"),
              GtIdentifier(GtSpan(9, 14), "Int16"),
              GtIdentifier(GtSpan(20, 25), "Int32"),
              GtIdentifier(GtSpan(31, 36), "Int64"),
              GtIdentifier(GtSpan(42, 48), "Int128"),
              GtIdentifier(GtSpan(55, 62), "IntSize"),
              GtIdentifier(GtSpan(70, 75), "IntU8"),
              GtIdentifier(GtSpan(80, 86), "IntU16"),
              GtIdentifier(GtSpan(92, 98), "IntU32"),
              GtIdentifier(GtSpan(104, 110), "IntU64"),
              GtIdentifier(GtSpan(116, 123), "IntU128"),
              GtIdentifier(GtSpan(130, 138), "IntUSize"),
              GtIdentifier(GtSpan(146, 153), "Float32"),
              GtIdentifier(GtSpan(159, 166), "Float64"),
              GtIdentifier(GtSpan(173, 183), "Int8Record"),
              GtIdentifier(GtSpan(202, 213), "Int16Record"),
              GtIdentifier(GtSpan(233, 244), "Int32Record"),
              GtIdentifier(GtSpan(264, 275), "Int64Record"),
              GtIdentifier(GtSpan(295, 307), "Int128Record"),
              GtIdentifier(GtSpan(328, 341), "IntSizeRecord"),
              GtIdentifier(GtSpan(363, 374), "IntU8Record"),
              GtIdentifier(GtSpan(393, 405), "IntU16Record"),
              GtIdentifier(GtSpan(425, 437), "IntU32Record"),
              GtIdentifier(GtSpan(457, 469), "IntU64Record"),
              GtIdentifier(GtSpan(489, 502), "IntU128Record"),
              GtIdentifier(GtSpan(523, 537), "IntUSizeRecord"),
              GtIdentifier(GtSpan(559, 572), "Float32Record"),
              GtIdentifier(GtSpan(592, 605), "Float64Record"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/17-number_sizes.type",
            source: "Int8: i8\nInt16: i16\nInt32: i32\nInt64: i64\nInt128: i128\nIntSize: isize\nIntU8: u8\nIntU16: u16\nIntU32: u32\nIntU64: u64\nIntU128: u128\nIntUSize: usize\nFloat32: f32\nFloat64: f64\n\nInt8Record: { [i8]: string }\nInt16Record: { [i16]: string }\nInt32Record: { [i32]: string }\nInt64Record: { [i64]: string }\nInt128Record: { [i128]: string }\nIntSizeRecord: { [isize]: string }\nIntU8Record: { [u8]: string }\nIntU16Record: { [u16]: string }\nIntU32Record: { [u32]: string }\nIntU64Record: { [u64]: string }\nIntU128Record: { [u128]: string }\nIntUSizeRecord: { [usize]: string }\nFloat32Record: { [f32]: string }\nFloat64Record: { [f64]: string }\n",
            language: None,
          ),
        )
        "#);
    }

    #[test]
    fn test_number() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/18-number.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Hello"),
                span: GtSpan(0, 13),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 5), "Hello"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(7, 13),
                  kind: Number,
                  doc: None,
                  attributes: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "World"),
                span: GtSpan(15, 42),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(15, 20), "World"),
                descriptor: Record(GtRecord(
                  span: GtSpan(22, 42),
                  doc: None,
                  attributes: [],
                  key: Number(GtSpan(24, 32)),
                  descriptor: Primitive(GtPrimitive(
                    span: GtSpan(34, 40),
                    kind: String,
                    doc: None,
                    attributes: [],
                  )),
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [],
            exports: [
              GtIdentifier(GtSpan(0, 5), "Hello"),
              GtIdentifier(GtSpan(15, 20), "World"),
            ],
            references: [],
          ),
          source_code: NamedSource(
            name: "../../examples/02-syntax/18-number.type",
            source: "Hello: number\n\nWorld: { [number]: string }",
            language: None,
          ),
        )
        "#);
    }

    fn parse_module(path: &str) -> GtModuleParse {
        let content = fs::read_to_string(path).expect("cannot read file");
        let source_code = NamedSource::new(path, content);
        GtModule::parse("module".into(), source_code).unwrap()
    }
}
