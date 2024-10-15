use pest::iterators::{Pair, Pairs};

use crate::*;

use super::GTAlias;

impl GTAlias {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let mut inner = pair.into_inner();

        match inner.next() {
            Some(pair) => parse(inner, pair, resolve, ParseState::Doc(span, None)),
            None => Err(GTNodeParseError::Internal(span, GTNode::Alias)),
        }
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    resolve: &mut GTResolve,
    state: ParseState,
) -> GTNodeParseResult<GTAlias> {
    match state {
        ParseState::Doc(span, doc_acc) => match pair.as_rule() {
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
                    Some(pair) => parse(inner, pair, resolve, ParseState::Doc(span, doc_acc)),
                    None => Err(GTNodeParseError::Internal(span, GTNode::Alias)),
                }
            }

            _ => parse(inner, pair, resolve, ParseState::Name(span, doc_acc)),
        },

        ParseState::Name(span, doc) => {
            let name: GTIdentifier = pair.into();
            resolve.exports.push(name.clone());

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    resolve,
                    ParseState::Descriptor(span, doc, name),
                ),
                None => Err(GTNodeParseError::Internal(span, GTNode::Alias)),
            }
        }

        ParseState::Descriptor(span, doc, name) => {
            let descriptor = GTDescriptor::parse(pair, resolve)?;
            Ok(GTAlias {
                span,
                doc,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Doc(GTSpan, Option<GTDoc>),
    Name(GTSpan, Option<GTDoc>),
    Descriptor(GTSpan, Option<GTDoc>, GTIdentifier),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello = { world: string }").unwrap();
        let mut resove = GTResolve::new();
        assert_eq!(
            GTAlias::parse(pairs.next().unwrap(), &mut resove).unwrap(),
            GTAlias {
                span: (0, 25).into(),
                name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                doc: None,
                descriptor: GTObject {
                    span: (8, 25).into(),
                    extensions: vec![],
                    properties: vec![GTProperty {
                        span: (10, 23).into(),
                        doc: None,
                        name: GTKey((10, 15).into(), "world".into()),
                        descriptor: GTPrimitive::String((17, 23).into()).into(),
                        required: true,
                    }]
                }
                .into()
            }
        );
    }

    #[test]
    fn test_parse_exports() {
        let source_code = crate::GTSourceCode::new("module.type".into(), "Hello = string".into());
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(
            parse.resolve.exports,
            vec![GTIdentifier::new((0, 5).into(), "Hello".into())]
        );
    }
}
