use pest::iterators::{Pair, Pairs};

use crate::*;

impl GTImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let else_err = || GTParseError::Internal(span.clone(), GTNode::Import);

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
    context: &mut GTContext,
    state: ParseState,
) -> GTNodeParseResult<GTImport> {
    match state {
        ParseState::Path(span) => {
            let (path, _) = GTPath::split_parse(pair)?;
            context.resolve.deps.insert(path.clone());

            match inner.next() {
                Some(pair) => parse(inner, pair, context, ParseState::Names(span, path)),
                None => Err(GTParseError::Internal(span.clone(), GTNode::Import)),
            }
        }

        ParseState::Names(span, path) => {
            let ref_span = pair.as_span().into();

            match pair.as_rule() {
                Rule::import_glob => Ok(GTImport {
                    span,
                    path,
                    reference: GTImportReference::Glob(ref_span),
                }),

                Rule::import_names => {
                    let mut names = vec![];

                    for pair in pair.into_inner() {
                        let name_span = pair.as_span().into();
                        let mut inner = pair.into_inner();

                        let name = inner
                            .next()
                            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Import))?
                            .into();

                        if let Some(alias) = inner.next() {
                            names.push(GTImportName::Alias(name_span, name, alias.into()));
                        } else {
                            names.push(GTImportName::Name(name_span, name));
                        }
                    }

                    Ok(GTImport {
                        span,
                        path,
                        reference: GTImportReference::Names(ref_span, names),
                    })
                }

                Rule::name => Ok(GTImport {
                    span,
                    path,
                    reference: GTImportReference::Name(ref_span, pair.into()),
                }),

                _ => Err(GTParseError::Internal(span, GTNode::Import)),
            }
        }
    }
}

enum ParseState {
    Path(GTSpan),
    Names(GTSpan, GTPath),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::import, "use ./hello/World").unwrap();
        assert_eq!(
            GTImport::parse(pairs.next().unwrap(), &mut GTContext::new()).unwrap(),
            GTImport {
                span: (0, 17).into(),
                path: GTPath::parse((4, 11).into(), "./hello").unwrap(),
                reference: GTImportReference::Name(
                    (12, 17).into(),
                    GTIdentifier::new((12, 17).into(), "World".into())
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
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec![
                GTPath::parse((4, 10).into(), "author").unwrap(),
                GTPath::parse((29, 36).into(), "../user").unwrap(),
                GTPath::parse((58, 70).into(), "./misc/order").unwrap()
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
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec![
                GTPath::parse((4, 12).into(), "author").unwrap(),
                GTPath::parse((31, 46).into(), "../user").unwrap(),
                GTPath::parse((68, 84).into(), "./misc/order").unwrap(),
            ])
        );
    }
}
