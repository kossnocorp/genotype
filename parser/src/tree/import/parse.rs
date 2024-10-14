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
            let path = pair.as_str();
            // Remove trailing slash
            let path = GTPath::new(path[..path.len() - 1].into());
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

    use crate::{tree::GTModule, GTSourceCode};

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
                "author".into(),
                "../user".into(),
                "./misc/order".into()
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
                "author".into(),
                "../user".into(),
                "./misc/order".into(),
            ])
        );
    }
}
