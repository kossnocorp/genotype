use pest::iterators::{Pair, Pairs};

use crate::*;

use super::GTProperty;

impl GTProperty {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let required = pair.as_rule() == Rule::required_property;
        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), GTNode::Property))?;
        parse(inner, pair, context, ParseState::Doc(span, required, None))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GTContext,
    state: ParseState,
) -> GTNodeParseResult<GTProperty> {
    match state {
        ParseState::Doc(span, required, doc_acc) => match pair.as_rule() {
            Rule::line_doc => {
                let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                let doc_acc = if let Some(pair) = doc {
                    Some(if let Some(doc) = doc_acc {
                        doc.concat(pair)
                    } else {
                        pair.into()
                    })
                } else {
                    doc_acc
                };

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Doc(span, required, doc_acc),
                    ),
                    None => Err(GTNodeParseError::Internal(span, GTNode::Property)),
                }
            }

            _ => parse(
                inner,
                pair,
                context,
                ParseState::Name(span, required, doc_acc),
            ),
        },

        ParseState::Name(span, required, doc) => {
            let name = GTKey::parse(pair);
            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, required, doc, name),
                ),
                None => Err(GTNodeParseError::Internal(span, GTNode::Property)),
            }
        }

        ParseState::Descriptor(span, required, doc, name) => {
            let descriptor = GTDescriptor::parse(pair, context)?;
            Ok(GTProperty {
                span,
                doc,
                name,
                descriptor,
                required,
            })
        }
    }
}

enum ParseState {
    Doc(GTSpan, bool, Option<GTDoc>),
    Name(GTSpan, bool, Option<GTDoc>),
    Descriptor(GTSpan, bool, Option<GTDoc>, GTKey),
}
