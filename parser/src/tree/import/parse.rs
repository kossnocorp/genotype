use pest::iterators::{Pair, Pairs};

use crate::*;

impl GTImport {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Result<Self, GTNodeParseError> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]

        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]

        let import = parse(inner, pair, resolve, ParseState::Path)?;

        Ok(import)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    resolve: &mut GTResolve,
    state: ParseState,
) -> Result<GTImport, GTNodeParseError> {
    match state {
        ParseState::Path => {
            let (path, _) = GTPath::split_parse(pair)?;

            resolve.deps.insert(path.clone());

            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, resolve, ParseState::Names(path))
        }

        ParseState::Names(path) => match pair.as_rule() {
            Rule::import_glob => Ok(GTImport {
                path,
                reference: GTImportReference::Glob,
            }),

            Rule::import_names => {
                let mut names = vec![];

                for pair in pair.into_inner() {
                    let mut inner = pair.into_inner();

                    let name = inner.next().unwrap().into();
                    let alias = inner.next();

                    if let Some(alias) = alias {
                        names.push(GTImportName::Alias(name, alias.into()));
                    } else {
                        names.push(GTImportName::Name(name));
                    }
                }

                Ok(GTImport {
                    path,
                    reference: GTImportReference::Names(names),
                })
            }

            Rule::name => Ok(GTImport {
                path,
                reference: GTImportReference::Name(pair.into()),
            }),

            _ => {
                println!("5 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        },
    }
}

enum ParseState {
    Path,
    Names(GTPath),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse_deps_base() {
        let source_code = GTSourceCode::new(
            "module.type".into(),
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
        let source_code = GTSourceCode::new(
            "module.type".into(),
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
