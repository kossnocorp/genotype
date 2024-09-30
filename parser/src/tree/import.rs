use pest::iterators::{Pair, Pairs};

use crate::parser::Rule;

#[derive(Debug, PartialEq, Clone)]
pub struct GTImport {
    pub path: String,
    pub reference: ImportReference,
}

pub fn parse_import(
    pair: Pair<'_, crate::parser::Rule>,
) -> Result<GTImport, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap(); // [TODO]

    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap(); // [TODO]

    let import = parse(inner, pair, ParseState::Path)?;

    Ok(import)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportReference {
    Glob,
    Names(Vec<ImportName>),
    Name(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportName {
    Name(String),
    Alias(String, String),
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> Result<GTImport, Box<dyn std::error::Error>> {
    match state {
        ParseState::Path => {
            let path = pair.as_str();
            // Remove trailing slash
            let path = path[..path.len() - 1].to_string();
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Names(path))
        }

        ParseState::Names(path) => match pair.as_rule() {
            Rule::import_glob => Ok(GTImport {
                path,
                reference: ImportReference::Glob,
            }),

            Rule::import_names => {
                let mut names = vec![];

                for pair in pair.into_inner() {
                    let mut inner = pair.into_inner();

                    let name = inner.next().unwrap().as_str().to_string();
                    let alias = inner.next();

                    if let Some(alias) = alias {
                        let alias = alias.as_str().to_string();
                        names.push(ImportName::Alias(name, alias));
                    } else {
                        names.push(ImportName::Name(name));
                    }
                }

                Ok(GTImport {
                    path,
                    reference: ImportReference::Names(names),
                })
            }

            Rule::name => {
                let name = pair.as_str().to_string();
                Ok(GTImport {
                    path,
                    reference: ImportReference::Name(name),
                })
            }

            _ => {
                println!("5 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        },
    }
}

enum ParseState {
    Path,
    Names(String),
}
