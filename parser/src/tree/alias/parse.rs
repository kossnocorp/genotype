use pest::iterators::{Pair, Pairs};

use crate::*;

use super::GTAlias;

impl GTAlias {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), GTNode::Alias))?;
        let alias = parse(inner, pair, context, ParseState::Doc(span.clone(), None))?;

        context.pop_parent(span, GTNode::Alias)?;

        Ok(alias)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GTContext,
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
                    Some(pair) => parse(inner, pair, context, ParseState::Doc(span, doc_acc)),
                    None => Err(GTNodeParseError::Internal(span, GTNode::Alias)),
                }
            }

            _ => parse(inner, pair, context, ParseState::Name(span, doc_acc)),
        },

        ParseState::Name(span, doc) => {
            let name: GTIdentifier = pair.into();

            context.resolve.exports.push(name.clone());
            context.parents.push(GTContextParent::Alias(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, doc, name),
                ),
                None => Err(GTNodeParseError::Internal(span, GTNode::Alias)),
            }
        }

        ParseState::Descriptor(span, doc, name) => {
            let descriptor = GTDescriptor::parse(pair, context)?;
            Ok(GTAlias {
                span,
                doc,
                attributes: vec![], // [TODO]
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
        assert_eq!(
            GTAlias::parse(pairs.next().unwrap(), &mut GTContext::new()).unwrap(),
            GTAlias {
                span: (0, 25).into(),
                name: GTIdentifier::new((0, 5).into(), "Hello".into()),
                doc: None,
                attributes: vec![],
                descriptor: GTObject {
                    span: (8, 25).into(),
                    name: GTIdentifier::new((0, 5).into(), "Hello".into()).into(),
                    extensions: vec![],
                    properties: vec![GTProperty {
                        span: (10, 23).into(),
                        doc: None,
                        attributes: vec![],
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

    #[test]
    fn test_parse_parent() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello = { world: string }").unwrap();
        let parents = vec![GTContextParent::Alias(GTIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GTContext {
            parents: parents.clone(),
            resolve: GTResolve::new(),
        };

        GTAlias::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
    }
}
