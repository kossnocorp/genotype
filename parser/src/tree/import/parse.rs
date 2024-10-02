use pest::iterators::{Pair, Pairs};

use crate::{
    parser::Rule,
    tree::{import_name::GTImportName, import_reference::GTImportReference, path::GTPath},
};

use super::GTImport;

impl TryFrom<Pair<'_, Rule>> for GTImport {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]

        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]

        let import = parse(inner, pair, ParseState::Path)?;

        Ok(import)
    }
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
            let path = GTPath(path[..path.len() - 1].into());
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Names(path))
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

                    let name = inner.next().unwrap().try_into()?;
                    let alias = inner.next();

                    if let Some(alias) = alias {
                        let alias = alias.try_into()?;
                        names.push(GTImportName::Alias(name, alias));
                    } else {
                        names.push(GTImportName::Name(name));
                    }
                }

                Ok(GTImport {
                    path,
                    reference: GTImportReference::Names(names),
                })
            }

            Rule::name => {
                let name = pair.try_into()?;
                Ok(GTImport {
                    path,
                    reference: GTImportReference::Name(name),
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
    Names(GTPath),
}
