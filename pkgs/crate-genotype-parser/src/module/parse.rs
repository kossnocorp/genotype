use crate::prelude::internal::*;

/// Module parse result. It contains the module tree and resolve data.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtModuleParse {
    /// Module tree.
    pub module: GtModule,
    /// Module resolve. It contains module meta information used to build
    /// the dependency graph.
    pub resolve: GtModuleResolve,
}

impl GtModule {
    /// Parse module from source code. It returns [GtModuleParse], wrapping the tree with
    /// the resolve data.
    pub fn parse(module_id: GtModuleId, source_code: &str) -> Result<GtModuleParse, GtParseError> {
        parse_gt_code(source_code)
            .map_err(|error| GtParseError::Syntax(error.into()))
            .and_then(|mut pairs| pairs.next().ok_or(GtParseError::InvalidGrammar))
            .and_then(|pair| Self::parse_root_token_pair(module_id.clone(), pair))
    }

    /// Parses root token pair into [GtModuleParse].
    fn parse_root_token_pair(
        module_id: GtModuleId,
        module_pair: Pair<'_, Rule>,
    ) -> Result<GtModuleParse, GtParseError> {
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
                    ));
                }
            }
        }

        Ok(GtModuleParse {
            module: GtModule {
                id: context.module_id.clone(),
                doc,
                imports,
                aliases,
            },
            resolve: context.resolve,
        })
    }
}

#[cfg(test)]
mod tests {
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
                generics: [],
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
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(22, 25),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(22, 25)),
                  identifier: GtIdentifier(GtSpan(22, 25), "Age"),
                  arguments: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "snake_case"),
                span: GtSpan(27, 42),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(27, 37), "snake_case"),
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                        generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                        arguments: [],
                        path: GtPath(
                          span: GtSpan(119, 132),
                          id: GtPathModuleId(
                            span: GtSpan(119, 132),
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
                        identifier: GtIdentifier(GtSpan(149, 154), "Genre"),
                        arguments: [],
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
                generics: [],
                descriptor: InlineImport(GtInlineImport(
                  span: GtSpan(167, 186),
                  doc: None,
                  attributes: [],
                  name: GtIdentifier(GtSpan(180, 186), "Author"),
                  arguments: [],
                  path: GtPath(
                    span: GtSpan(167, 180),
                    id: GtPathModuleId(
                      span: GtSpan(167, 180),
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
                generics: [],
                descriptor: Array(GtArray(
                  span: GtSpan(197, 218),
                  doc: None,
                  attributes: [],
                  descriptor: InlineImport(GtInlineImport(
                    span: GtSpan(198, 217),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(211, 217), "Author"),
                    arguments: [],
                    path: GtPath(
                      span: GtSpan(198, 211),
                      id: GtPathModuleId(
                        span: GtSpan(198, 211),
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
                span: GtSpan(119, 132),
                id: GtPathModuleId(
                  span: GtSpan(119, 132),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
              GtPath(
                span: GtSpan(167, 180),
                id: GtPathModuleId(
                  span: GtSpan(167, 180),
                  module_id: GtModuleId("module"),
                ),
                path: "../../author",
              ),
              GtPath(
                span: GtSpan(198, 211),
                id: GtPathModuleId(
                  span: GtSpan(198, 211),
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                        identifier: GtIdentifier(GtSpan(57, 61), "Base"),
                        arguments: [],
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
                generics: [],
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
                        identifier: GtIdentifier(GtSpan(93, 97), "Base"),
                        arguments: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                        identifier: GtIdentifier(GtSpan(62, 73), "CommentBase"),
                        arguments: [],
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
                generics: [],
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
                        identifier: GtIdentifier(GtSpan(156, 167), "CommentBase"),
                        arguments: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                      identifier: GtIdentifier(GtSpan(166, 181), "ObjectUnionUser"),
                      arguments: [],
                    )),
                    Reference(GtReference(
                      span: GtSpan(186, 204),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(186, 204)),
                      identifier: GtIdentifier(GtSpan(186, 204), "ObjectUnionAccount"),
                      arguments: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                      identifier: GtIdentifier(GtSpan(9, 14), "Reply"),
                      arguments: [],
                    )),
                    Reference(GtReference(
                      span: GtSpan(17, 19),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(17, 19)),
                      identifier: GtIdentifier(GtSpan(17, 19), "DM"),
                      arguments: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                      identifier: GtIdentifier(GtSpan(297, 312), "SuccessResponse"),
                      arguments: [],
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
                      identifier: GtIdentifier(GtSpan(331, 344), "ErrorResponse"),
                      arguments: [],
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
                generics: [],
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
                      identifier: GtIdentifier(GtSpan(380, 395), "SuccessResponse"),
                      arguments: [],
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
                      identifier: GtIdentifier(GtSpan(418, 431), "ErrorResponse"),
                      arguments: [],
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
                generics: [],
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
                      identifier: GtIdentifier(GtSpan(465, 480), "SuccessResponse"),
                      arguments: [],
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
                      identifier: GtIdentifier(GtSpan(503, 516), "ErrorResponse"),
                      arguments: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
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
                generics: [],
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
                generics: [],
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
            generic_parameters: [],
          ),
        )
        "#);
    }

    #[test]
    fn test_generics() {
        assert_ron_snapshot!(parse_module("../../examples/02-syntax/19-generics.type"), @r#"
        GtModuleParse(
          module: GtModule(
            id: GtModuleId("module"),
            doc: None,
            imports: [],
            aliases: [
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Response"),
                span: GtSpan(0, 67),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(0, 8), "Response"),
                generics: [
                  GtGenericParameter(
                    span: GtSpan(9, 16),
                    identifier: GtIdentifier(GtSpan(9, 16), "Payload"),
                  ),
                ],
                descriptor: Union(GtUnion(
                  span: GtSpan(21, 67),
                  doc: None,
                  attributes: [],
                  descriptors: [
                    Reference(GtReference(
                      span: GtSpan(23, 47),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(23, 47)),
                      identifier: GtIdentifier(GtSpan(23, 38), "ResponseSuccess"),
                      arguments: [
                        GtGenericArgument(
                          span: GtSpan(38, 47),
                          descriptor: Reference(GtReference(
                            span: GtSpan(39, 46),
                            doc: None,
                            attributes: [],
                            id: GtReferenceId(GtModuleId("module"), GtSpan(39, 46)),
                            identifier: GtIdentifier(GtSpan(39, 46), "Payload"),
                            arguments: [],
                          )),
                        ),
                      ],
                    )),
                    Reference(GtReference(
                      span: GtSpan(52, 67),
                      doc: None,
                      attributes: [],
                      id: GtReferenceId(GtModuleId("module"), GtSpan(52, 67)),
                      identifier: GtIdentifier(GtSpan(52, 67), "ResponseFailure"),
                      arguments: [],
                    )),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ResponseSuccess"),
                span: GtSpan(69, 137),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(69, 84), "ResponseSuccess"),
                generics: [
                  GtGenericParameter(
                    span: GtSpan(85, 92),
                    identifier: GtIdentifier(GtSpan(85, 92), "Payload"),
                  ),
                ],
                descriptor: Object(GtObject(
                  span: GtSpan(95, 137),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(69, 84), "ResponseSuccess")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(99, 116),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(99, 105), "status"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(107, 116),
                        doc: None,
                        attributes: [],
                        value: String("success"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(120, 134),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(120, 125), "value"),
                      descriptor: Reference(GtReference(
                        span: GtSpan(127, 134),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(127, 134)),
                        identifier: GtIdentifier(GtSpan(127, 134), "Payload"),
                        arguments: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ResponseFailure"),
                span: GtSpan(139, 197),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(139, 154), "ResponseFailure"),
                generics: [],
                descriptor: Object(GtObject(
                  span: GtSpan(156, 197),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(139, 154), "ResponseFailure")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(160, 177),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(160, 166), "status"),
                      descriptor: Literal(GtLiteral(
                        span: GtSpan(168, 177),
                        doc: None,
                        attributes: [],
                        value: String("failure"),
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(181, 194),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(181, 186), "error"),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(188, 194),
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
                id: GtDefinitionId(GtModuleId("module"), "ResponseReadFile"),
                span: GtSpan(199, 233),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(199, 215), "ResponseReadFile"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(217, 233),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(217, 233)),
                  identifier: GtIdentifier(GtSpan(217, 225), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(225, 233),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(226, 232),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ResponseGlob"),
                span: GtSpan(235, 267),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(235, 247), "ResponseGlob"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(249, 267),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(249, 267)),
                  identifier: GtIdentifier(GtSpan(249, 257), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(257, 267),
                      descriptor: Array(GtArray(
                        span: GtSpan(258, 266),
                        doc: None,
                        attributes: [],
                        descriptor: Primitive(GtPrimitive(
                          span: GtSpan(259, 265),
                          kind: String,
                          doc: None,
                          attributes: [],
                        )),
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ResponseIsFile"),
                span: GtSpan(269, 302),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(269, 283), "ResponseIsFile"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(285, 302),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(285, 302)),
                  identifier: GtIdentifier(GtSpan(285, 293), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(293, 302),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(294, 301),
                        kind: Boolean,
                        doc: None,
                        attributes: [],
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "ResponseFindFile"),
                span: GtSpan(304, 345),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(304, 320), "ResponseFindFile"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(322, 345),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(322, 345)),
                  identifier: GtIdentifier(GtSpan(322, 330), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(330, 345),
                      descriptor: Union(GtUnion(
                        span: GtSpan(330, 345),
                        doc: None,
                        attributes: [],
                        descriptors: [
                          Primitive(GtPrimitive(
                            span: GtSpan(331, 337),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Literal(GtLiteral(
                            span: GtSpan(340, 344),
                            doc: None,
                            attributes: [],
                            value: Null,
                          )),
                        ],
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "InlineImportGeneric"),
                span: GtSpan(347, 394),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(347, 366), "InlineImportGeneric"),
                generics: [],
                descriptor: InlineImport(GtInlineImport(
                  span: GtSpan(368, 394),
                  doc: None,
                  attributes: [],
                  name: GtIdentifier(GtSpan(378, 386), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(386, 394),
                      descriptor: Primitive(GtPrimitive(
                        span: GtSpan(387, 393),
                        kind: String,
                        doc: None,
                        attributes: [],
                      )),
                    ),
                  ],
                  path: GtPath(
                    span: GtSpan(368, 378),
                    id: GtPathModuleId(
                      span: GtSpan(368, 378),
                      module_id: GtModuleId("module"),
                    ),
                    path: "./runtime",
                  ),
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "Pair"),
                span: GtSpan(396, 448),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(396, 400), "Pair"),
                generics: [
                  GtGenericParameter(
                    span: GtSpan(401, 405),
                    identifier: GtIdentifier(GtSpan(401, 405), "Left"),
                  ),
                  GtGenericParameter(
                    span: GtSpan(407, 412),
                    identifier: GtIdentifier(GtSpan(407, 412), "Right"),
                  ),
                ],
                descriptor: Object(GtObject(
                  span: GtSpan(415, 448),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(396, 400), "Pair")),
                  extensions: [],
                  properties: [
                    GtProperty(
                      span: GtSpan(419, 429),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(419, 423), "left"),
                      descriptor: Reference(GtReference(
                        span: GtSpan(425, 429),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(425, 429)),
                        identifier: GtIdentifier(GtSpan(425, 429), "Left"),
                        arguments: [],
                      )),
                      required: true,
                    ),
                    GtProperty(
                      span: GtSpan(433, 445),
                      doc: None,
                      attributes: [],
                      name: GtKey(GtSpan(433, 438), "right"),
                      descriptor: Reference(GtReference(
                        span: GtSpan(440, 445),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(440, 445)),
                        identifier: GtIdentifier(GtSpan(440, 445), "Right"),
                        arguments: [],
                      )),
                      required: true,
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "StringNumberPair"),
                span: GtSpan(450, 488),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(450, 466), "StringNumberPair"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(468, 488),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(468, 488)),
                  identifier: GtIdentifier(GtSpan(468, 472), "Pair"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(472, 488),
                      descriptor: Union(GtUnion(
                        span: GtSpan(472, 488),
                        doc: None,
                        attributes: [],
                        descriptors: [
                          Primitive(GtPrimitive(
                            span: GtSpan(473, 479),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          Primitive(GtPrimitive(
                            span: GtSpan(481, 487),
                            kind: Number,
                            doc: None,
                            attributes: [],
                          )),
                        ],
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "WithExt"),
                span: GtSpan(490, 529),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(490, 497), "WithExt"),
                generics: [],
                descriptor: Object(GtObject(
                  span: GtSpan(499, 529),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(490, 497), "WithExt")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(503, 526),
                      reference: GtReference(
                        span: GtSpan(506, 526),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(506, 526)),
                        identifier: GtIdentifier(GtSpan(506, 510), "Pair"),
                        arguments: [
                          GtGenericArgument(
                            span: GtSpan(510, 526),
                            descriptor: Union(GtUnion(
                              span: GtSpan(510, 526),
                              doc: None,
                              attributes: [],
                              descriptors: [
                                Primitive(GtPrimitive(
                                  span: GtSpan(511, 517),
                                  kind: String,
                                  doc: None,
                                  attributes: [],
                                )),
                                Primitive(GtPrimitive(
                                  span: GtSpan(519, 525),
                                  kind: Number,
                                  doc: None,
                                  attributes: [],
                                )),
                              ],
                            )),
                          ),
                        ],
                      ),
                    ),
                  ],
                  properties: [],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "WithInlineImport"),
                span: GtSpan(531, 586),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(531, 547), "WithInlineImport"),
                generics: [],
                descriptor: Reference(GtReference(
                  span: GtSpan(549, 586),
                  doc: None,
                  attributes: [],
                  id: GtReferenceId(GtModuleId("module"), GtSpan(549, 586)),
                  identifier: GtIdentifier(GtSpan(549, 557), "Response"),
                  arguments: [
                    GtGenericArgument(
                      span: GtSpan(557, 586),
                      descriptor: InlineImport(GtInlineImport(
                        span: GtSpan(558, 585),
                        doc: None,
                        attributes: [],
                        name: GtIdentifier(GtSpan(565, 569), "Pair"),
                        arguments: [
                          GtGenericArgument(
                            span: GtSpan(569, 585),
                            descriptor: Union(GtUnion(
                              span: GtSpan(569, 585),
                              doc: None,
                              attributes: [],
                              descriptors: [
                                Primitive(GtPrimitive(
                                  span: GtSpan(570, 576),
                                  kind: String,
                                  doc: None,
                                  attributes: [],
                                )),
                                Primitive(GtPrimitive(
                                  span: GtSpan(578, 584),
                                  kind: Number,
                                  doc: None,
                                  attributes: [],
                                )),
                              ],
                            )),
                          ),
                        ],
                        path: GtPath(
                          span: GtSpan(558, 565),
                          id: GtPathModuleId(
                            span: GtSpan(558, 565),
                            module_id: GtModuleId("module"),
                          ),
                          path: "./pair",
                        ),
                      )),
                    ),
                  ],
                )),
              ),
              GtAlias(
                id: GtDefinitionId(GtModuleId("module"), "WithInlineImportExt"),
                span: GtSpan(588, 690),
                doc: None,
                attributes: [],
                name: GtIdentifier(GtSpan(588, 607), "WithInlineImportExt"),
                generics: [],
                descriptor: Object(GtObject(
                  span: GtSpan(609, 690),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(588, 607), "WithInlineImportExt")),
                  extensions: [
                    GtExtension(
                      span: GtSpan(613, 653),
                      reference: GtReference(
                        span: GtSpan(616, 653),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(616, 653)),
                        identifier: GtIdentifier(GtSpan(616, 624), "Response"),
                        arguments: [
                          GtGenericArgument(
                            span: GtSpan(624, 653),
                            descriptor: InlineImport(GtInlineImport(
                              span: GtSpan(625, 652),
                              doc: None,
                              attributes: [],
                              name: GtIdentifier(GtSpan(632, 636), "Pair"),
                              arguments: [
                                GtGenericArgument(
                                  span: GtSpan(636, 652),
                                  descriptor: Union(GtUnion(
                                    span: GtSpan(636, 652),
                                    doc: None,
                                    attributes: [],
                                    descriptors: [
                                      Primitive(GtPrimitive(
                                        span: GtSpan(637, 643),
                                        kind: String,
                                        doc: None,
                                        attributes: [],
                                      )),
                                      Primitive(GtPrimitive(
                                        span: GtSpan(645, 651),
                                        kind: Number,
                                        doc: None,
                                        attributes: [],
                                      )),
                                    ],
                                  )),
                                ),
                              ],
                              path: GtPath(
                                span: GtSpan(625, 632),
                                id: GtPathModuleId(
                                  span: GtSpan(625, 632),
                                  module_id: GtModuleId("module"),
                                ),
                                path: "./pair",
                              ),
                            )),
                          ),
                        ],
                      ),
                    ),
                    GtExtension(
                      span: GtSpan(657, 687),
                      reference: GtReference(
                        span: GtSpan(660, 687),
                        doc: None,
                        attributes: [],
                        id: GtReferenceId(GtModuleId("module"), GtSpan(660, 687)),
                        identifier: GtIdentifier(GtSpan(667, 671), "Pair"),
                        arguments: [
                          GtGenericArgument(
                            span: GtSpan(671, 687),
                            descriptor: Union(GtUnion(
                              span: GtSpan(671, 687),
                              doc: None,
                              attributes: [],
                              descriptors: [
                                Primitive(GtPrimitive(
                                  span: GtSpan(672, 678),
                                  kind: String,
                                  doc: None,
                                  attributes: [],
                                )),
                                Primitive(GtPrimitive(
                                  span: GtSpan(680, 686),
                                  kind: Number,
                                  doc: None,
                                  attributes: [],
                                )),
                              ],
                            )),
                          ),
                        ],
                      ),
                    ),
                  ],
                  properties: [],
                )),
              ),
            ],
          ),
          resolve: GtModuleResolve(
            deps: [
              GtPath(
                span: GtSpan(368, 378),
                id: GtPathModuleId(
                  span: GtSpan(368, 378),
                  module_id: GtModuleId("module"),
                ),
                path: "./runtime",
              ),
              GtPath(
                span: GtSpan(558, 565),
                id: GtPathModuleId(
                  span: GtSpan(558, 565),
                  module_id: GtModuleId("module"),
                ),
                path: "./pair",
              ),
              GtPath(
                span: GtSpan(625, 632),
                id: GtPathModuleId(
                  span: GtSpan(625, 632),
                  module_id: GtModuleId("module"),
                ),
                path: "./pair",
              ),
              GtPath(
                span: GtSpan(660, 667),
                id: GtPathModuleId(
                  span: GtSpan(660, 667),
                  module_id: GtModuleId("module"),
                ),
                path: "./pair",
              ),
            ],
            exports: [
              GtIdentifier(GtSpan(0, 8), "Response"),
              GtIdentifier(GtSpan(69, 84), "ResponseSuccess"),
              GtIdentifier(GtSpan(139, 154), "ResponseFailure"),
              GtIdentifier(GtSpan(199, 215), "ResponseReadFile"),
              GtIdentifier(GtSpan(235, 247), "ResponseGlob"),
              GtIdentifier(GtSpan(269, 283), "ResponseIsFile"),
              GtIdentifier(GtSpan(304, 320), "ResponseFindFile"),
              GtIdentifier(GtSpan(347, 366), "InlineImportGeneric"),
              GtIdentifier(GtSpan(396, 400), "Pair"),
              GtIdentifier(GtSpan(450, 466), "StringNumberPair"),
              GtIdentifier(GtSpan(490, 497), "WithExt"),
              GtIdentifier(GtSpan(531, 547), "WithInlineImport"),
              GtIdentifier(GtSpan(588, 607), "WithInlineImportExt"),
            ],
            references: [
              GtIdentifier(GtSpan(39, 46), "Payload"),
              GtIdentifier(GtSpan(23, 38), "ResponseSuccess"),
              GtIdentifier(GtSpan(52, 67), "ResponseFailure"),
              GtIdentifier(GtSpan(127, 134), "Payload"),
              GtIdentifier(GtSpan(217, 225), "Response"),
              GtIdentifier(GtSpan(249, 257), "Response"),
              GtIdentifier(GtSpan(285, 293), "Response"),
              GtIdentifier(GtSpan(322, 330), "Response"),
              GtIdentifier(GtSpan(425, 429), "Left"),
              GtIdentifier(GtSpan(440, 445), "Right"),
              GtIdentifier(GtSpan(468, 472), "Pair"),
              GtIdentifier(GtSpan(506, 510), "Pair"),
              GtIdentifier(GtSpan(549, 557), "Response"),
              GtIdentifier(GtSpan(616, 624), "Response"),
              GtIdentifier(GtSpan(667, 671), "Pair"),
            ],
            generic_parameters: [
              GtIdentifier(GtSpan(39, 46), "Payload"),
              GtIdentifier(GtSpan(127, 134), "Payload"),
              GtIdentifier(GtSpan(425, 429), "Left"),
              GtIdentifier(GtSpan(440, 445), "Right"),
            ],
          ),
        )
        "#);
    }

    fn parse_module(path: &str) -> GtModuleParse {
        let source_code = fs::read_to_string(path).expect("cannot read file");
        GtModule::parse("module".into(), &source_code).unwrap()
    }
}
