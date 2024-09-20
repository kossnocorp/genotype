use pest::iterators::{Pair, Pairs};

use crate::parser::Rule;

#[derive(Debug, PartialEq)]
pub struct Import {
    pub path: String,
    pub reference: ImportReference,
}

pub fn parse_import(
    pair: Pair<'_, crate::parser::Rule>,
) -> Result<Import, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap(); // [TODO]

    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap(); // [TODO]

    let import = parse(inner, pair, ParseState::Path)?;

    Ok(import)
}

#[derive(Debug, PartialEq)]
pub enum ImportReference {
    Glob,
    Names(Vec<ImportName>),
    Name(String),
}

#[derive(Debug, PartialEq)]
pub enum ImportName {
    Name(String),
    Alias(String, String),
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> Result<Import, Box<dyn std::error::Error>> {
    match state {
        ParseState::Path => {
            let path = pair.as_str().to_string();
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Names(path))
        }

        ParseState::Names(path) => match pair.as_rule() {
            Rule::import_glob => Ok(Import {
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

                Ok(Import {
                    path,
                    reference: ImportReference::Names(names),
                })
            }

            Rule::name => {
                let name = pair.as_str().to_string();
                Ok(Import {
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
