use pest::iterators::{Pair, Pairs};

use crate::{
    parser::Rule,
    tree::{doc::GTDoc, key::GTKey, GTDescriptor, GTResolve},
};

use super::GTProperty;

impl GTProperty {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let required = pair.as_rule() == Rule::required_property;
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]
        parse(inner, pair, resolve, ParseState::Doc(required, None))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    resolve: &mut GTResolve,
    state: ParseState,
) -> Result<GTProperty, Box<dyn std::error::Error>> {
    match state {
        ParseState::Doc(required, doc_acc) => {
            match pair.as_rule() {
                Rule::line_doc => {
                    let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                    let doc_acc = if let Some(pair) = doc {
                        Some(if let Some(doc) = doc_acc {
                            doc.concat(pair)
                        } else {
                            GTDoc::parse(pair)
                        })
                    } else {
                        doc_acc
                    };

                    let pair = inner.next().unwrap(); // [TODO]
                    parse(inner, pair, resolve, ParseState::Doc(required, doc_acc))
                }

                _ => parse(inner, pair, resolve, ParseState::Name(required, doc_acc)),
            }
        }

        ParseState::Name(required, doc) => {
            let name = GTKey::parse(pair);
            let pair = inner.next().unwrap(); // [TODO]
            parse(
                inner,
                pair,
                resolve,
                ParseState::Descriptor(required, doc, name),
            )
        }

        ParseState::Descriptor(required, doc, name) => {
            let descriptor = GTDescriptor::parse(pair, resolve)?;
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
    Doc(bool, Option<GTDoc>),
    Name(bool, Option<GTDoc>),
    Descriptor(bool, Option<GTDoc>, GTKey),
}
