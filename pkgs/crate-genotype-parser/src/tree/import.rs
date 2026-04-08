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
        let else_err = || GtParseError::Internal(span, GtNode::Import);

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(else_err)?;

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(else_err)?;

        let import = parse(inner, pair, context, ParseState::Path(span))?;

        Ok(import)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtImport> {
    match state {
        ParseState::Path(span) => {
            let (path, _) = GtPath::split_parse(pair, &context.module_id)?;
            context.resolve.deps.insert(path.clone());

            match inner.next() {
                Some(pair) => parse(inner, pair, context, ParseState::Names(span, path)),
                None => Err(GtParseError::Internal(span, GtNode::Import)),
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
                            .ok_or(GtParseError::Internal(span, GtNode::Import))?
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

                _ => Err(GtParseError::Internal(span, GtNode::Import)),
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
    use indexmap::IndexSet;
    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

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
        let source_code = NamedSource::new(
            "module.type",
            r#"use author/*
            use ../user/User
            use ./misc/order/{Order, SomethingElse}"#
                .into(),
        );
        let parse = GtModule::parse("module".into(), source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            IndexSet::<_, std::collections::hash_map::RandomState>::from_iter(vec![
                GtPath::parse((4, 10).into(), &"module".into(), "author").unwrap(),
                GtPath::parse((29, 36).into(), &"module".into(), "../user").unwrap(),
                GtPath::parse((58, 70).into(), &"module".into(), "./misc/order").unwrap()
            ])
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = NamedSource::new(
            "module.type",
            r#"use author/./*
            use ../user/../user/User
            use ./././misc/order/{Order, SomethingElse}"#
                .into(),
        );
        let parse = GtModule::parse("module".into(), source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            IndexSet::<_, std::collections::hash_map::RandomState>::from_iter(vec![
                GtPath::parse((4, 12).into(), &"module".into(), "author").unwrap(),
                GtPath::parse((31, 46).into(), &"module".into(), "../user").unwrap(),
                GtPath::parse((68, 84).into(), &"module".into(), "./misc/order").unwrap(),
            ])
        );
    }
}
