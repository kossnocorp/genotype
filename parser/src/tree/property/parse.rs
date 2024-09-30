use pest::iterators::{Pair, Pairs};

use crate::{parser::Rule, tree::name::GTName};

use super::GTProperty;

impl TryFrom<Pair<'_, Rule>> for GTProperty {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let required = pair.as_rule() == Rule::required_property;
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]
        parse(inner, pair, ParseState::Doc(required, None))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> Result<GTProperty, Box<dyn std::error::Error>> {
    match state {
        ParseState::Doc(required, doc_acc) => {
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
                    parse(inner, pair, ParseState::Doc(required, doc_acc))
                }

                _ => parse(inner, pair, ParseState::Name(required, doc_acc)),
            }
        }

        ParseState::Name(required, doc) => {
            let name = GTName(pair.as_str().to_string()); // [TODO]
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Descriptor(required, doc, name))
        }

        ParseState::Descriptor(required, doc, name) => {
            let descriptor = pair.try_into()?;
            Ok(GTProperty {
                doc,
                name,
                descriptor,
                required,
            })
        }
    }
}

enum ParseState {
    Doc(bool, Option<String>),
    Name(bool, Option<String>),
    Descriptor(bool, Option<String>, GTName),
}
