use pest::iterators::{Pair, Pairs};

use crate::{parser::Rule, tree::name::GTName};

use super::GTAlias;

impl TryFrom<Pair<'_, Rule>> for GTAlias {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]
        parse(inner, pair, ParseState::Doc(None))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> Result<GTAlias, Box<dyn std::error::Error>> {
    match state {
        ParseState::Doc(doc_acc) => {
            match pair.as_rule() {
                Rule::line_doc => {
                    let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                    let doc_acc = if let Some(pair) = doc {
                        Some(if let Some(str) = doc_acc {
                            str + "\n" + pair.as_str()
                        } else {
                            pair.as_str().to_string()
                        })
                    } else {
                        doc_acc
                    };

                    let pair = inner.next().unwrap(); // [TODO]
                    parse(inner, pair, ParseState::Doc(doc_acc))
                }

                _ => parse(inner, pair, ParseState::Name(doc_acc)),
            }
        }

        ParseState::Name(doc) => {
            let name = GTName(pair.as_str().to_string());
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Descriptor(doc, name))
        }

        ParseState::Descriptor(doc, name) => {
            let descriptor = pair.try_into()?;
            Ok(GTAlias {
                doc,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Doc(Option<String>),
    Name(Option<String>),
    Descriptor(Option<String>, GTName),
}