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
        let property = parse(
            inner,
            pair,
            context,
            ParseState::Doc(span.clone(), required, None),
        )?;

        context.pop_parent(span, GTNode::Property)?;

        Ok(property)
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

            context
                .parents
                .push(GTContextParent::Property(name.clone()));

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

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::any_property, "world: string").unwrap();
        assert_eq!(
            GTProperty::parse(pairs.next().unwrap(), &mut GTContext::new()).unwrap(),
            GTProperty {
                span: (0, 13).into(),
                doc: None,
                name: GTKey::new((0, 5).into(), "world".into()),
                descriptor: GTPrimitive::String((7, 13).into()).into(),
                required: true
            }
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut pairs = GenotypeParser::parse(Rule::any_property, "world: string").unwrap();
        let parents = vec![GTContextParent::Alias(GTIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GTContext {
            parents: parents.clone(),
            resolve: GTResolve::new(),
        };

        GTProperty::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
    }
}
