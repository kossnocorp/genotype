use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtImport {
    pub span: GtSpan,
    #[visit]
    pub path: GtPath,
    #[visit]
    pub reference: GtImportReference,
}

impl GtImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let else_err = || GtParseError::UnexpectedEnd(span, GtNode::Import, "import inner");

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(else_err)?;

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(else_err)?;

        let import = parse(&span, inner, pair, context, ParseState::Path(span))?;

        Ok(import)
    }
}

fn parse(
    import_span: &GtSpan,
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtImport> {
    match state {
        ParseState::Path(span) => {
            let (path, _) = GtPath::split_parse(pair, &context.module_id)?;
            context
                .resolve
                .deps
                .insert(GtModuleSource::new(import_span, &path));

            match inner.next() {
                Some(pair) => parse(
                    import_span,
                    inner,
                    pair,
                    context,
                    ParseState::Names(span, path),
                ),
                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Import,
                    "import reference",
                )),
            }
        }

        ParseState::Names(span, path) => {
            let ref_span = pair.as_span().into();

            match pair.as_rule() {
                Rule::import_glob => Ok(GtImport {
                    span,
                    path,
                    reference: GtImportReference::Glob(ref_span),
                }),

                Rule::import_names => {
                    let mut names = vec![];

                    for pair in pair.into_inner() {
                        let name_span = pair.as_span().into();
                        let mut inner = pair.into_inner();

                        let name = inner
                            .next()
                            .ok_or(GtParseError::UnexpectedEnd(
                                span,
                                GtNode::Import,
                                "import name",
                            ))?
                            .into();

                        if let Some(alias) = inner.next() {
                            names.push(GtImportName::Alias(name_span, name, alias.into()));
                        } else {
                            names.push(GtImportName::Name(name_span, name));
                        }
                    }

                    Ok(GtImport {
                        span,
                        path,
                        reference: GtImportReference::Names(ref_span, names),
                    })
                }

                Rule::name => Ok(GtImport {
                    span,
                    path,
                    reference: GtImportReference::Name(ref_span, pair.into()),
                }),

                rule => Err(GtParseError::UnexpectedRule(
                    span,
                    GtNode::Import,
                    rule,
                    "expected import reference",
                )),
            }
        }
    }
}

enum ParseState {
    Path(GtSpan),
    Names(GtSpan, GtPath),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::import, "use ./hello/World").unwrap();
        assert_eq!(
            GtImport::parse(pairs.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            GtImport {
                span: (0, 17).into(),
                path: GtPath::parse((4, 11).into(), &"module".into(), "./hello").unwrap(),
                reference: GtImportReference::Name(
                    (12, 17).into(),
                    GtIdentifier::new((12, 17).into(), "World".into())
                )
            }
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = r#"use author/*
            use ../user/User
            use ./misc/order/{Order, SomethingElse}"#
            .to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GtModuleSource(
            span: GtSpan(0, 12),
            path: GtPath(
              span: GtSpan(4, 10),
              id: GtPathModuleId(
                span: GtSpan(4, 10),
                module_id: GtModuleId("module"),
              ),
              path: "author",
            ),
          ),
          GtModuleSource(
            span: GtSpan(25, 41),
            path: GtPath(
              span: GtSpan(29, 36),
              id: GtPathModuleId(
                span: GtSpan(29, 36),
                module_id: GtModuleId("module"),
              ),
              path: "../user",
            ),
          ),
          GtModuleSource(
            span: GtSpan(54, 93),
            path: GtPath(
              span: GtSpan(58, 70),
              id: GtPathModuleId(
                span: GtSpan(58, 70),
                module_id: GtModuleId("module"),
              ),
              path: "./misc/order",
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = r#"use author/./*
            use ../user/../user/User
            use ./././misc/order/{Order, SomethingElse}"#
            .to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GtModuleSource(
            span: GtSpan(0, 14),
            path: GtPath(
              span: GtSpan(4, 12),
              id: GtPathModuleId(
                span: GtSpan(4, 12),
                module_id: GtModuleId("module"),
              ),
              path: "author",
            ),
          ),
          GtModuleSource(
            span: GtSpan(27, 51),
            path: GtPath(
              span: GtSpan(31, 46),
              id: GtPathModuleId(
                span: GtSpan(31, 46),
                module_id: GtModuleId("module"),
              ),
              path: "../user",
            ),
          ),
          GtModuleSource(
            span: GtSpan(64, 107),
            path: GtPath(
              span: GtSpan(68, 84),
              id: GtPathModuleId(
                span: GtSpan(68, 84),
                module_id: GtModuleId("module"),
              ),
              path: "./misc/order",
            ),
          ),
        ]
        "#
        );
    }
}
