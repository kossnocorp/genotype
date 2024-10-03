use pest::iterators::{Pair, Pairs};

use crate::{
    parser::Rule,
    tree::{doc::GTDoc, identifier::GTIdentifier, GTDescriptor, GTResolve},
};

use super::GTAlias;

impl GTAlias {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap(); // [TODO]
        parse(inner, pair, resolve, ParseState::Doc(None))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    resolve: &mut GTResolve,
    state: ParseState,
) -> Result<GTAlias, Box<dyn std::error::Error>> {
    match state {
        ParseState::Doc(doc_acc) => {
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
                    parse(inner, pair, resolve, ParseState::Doc(doc_acc))
                }

                _ => parse(inner, pair, resolve, ParseState::Name(doc_acc)),
            }
        }

        ParseState::Name(doc) => {
            let name = GTIdentifier::parse(pair);
            let pair = inner.next().unwrap(); // [TODO]
            parse(inner, pair, resolve, ParseState::Descriptor(doc, name))
        }

        ParseState::Descriptor(doc, name) => {
            let descriptor = GTDescriptor::parse(pair, resolve)?;
            Ok(GTAlias {
                doc,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Doc(Option<GTDoc>),
    Name(Option<GTDoc>),
    Descriptor(Option<GTDoc>, GTIdentifier),
}
