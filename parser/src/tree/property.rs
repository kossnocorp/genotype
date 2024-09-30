use super::descriptor::{parse_descriptor, GTDescriptor};
use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProperty {
    pub doc: Option<String>,
    pub name: String,
    pub descriptor: GTDescriptor,
    pub required: bool,
}

pub fn parse_property(pair: Pair<'_, Rule>) -> Result<GTProperty, Box<dyn std::error::Error>> {
    let required = pair.as_rule() == Rule::required_property;
    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap(); // [TODO]
    parse(inner, pair, ParseState::Doc(required, None))
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
            let name = Some(pair.as_str().to_string()).unwrap(); // [TODO]
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, ParseState::Descriptor(required, doc, name))
        }

        ParseState::Descriptor(required, doc, name) => {
            let descriptor = parse_descriptor(pair)?;
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
    Descriptor(bool, Option<String>, String),
}
