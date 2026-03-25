use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTProperty {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub name: GTKey,
    #[visit]
    pub descriptor: GTDescriptor,
    pub required: bool,
}

impl GTProperty {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let required = pair.as_rule() == Rule::required_property;
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Property))?;
        let property = parse(
            inner,
            pair,
            ParseState::Doc(span.clone(), required, None),
            context,
        )?;

        context.exit_parent(span, GTNode::Property)?;

        Ok(property)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GTContext,
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
                        ParseState::Doc(span, required, doc_acc),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Property)),
                }
            }

            _ => parse(
                inner,
                pair,
                ParseState::Attributes(span, required, doc_acc, vec![]),
                context,
            ),
        },

        ParseState::Attributes(span, required, doc, mut attributes) => match pair.as_rule() {
            Rule::attribute => {
                let attribute = GTAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Attributes(span, required, doc, attributes),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Property)),
                }
            }

            _ => parse(
                inner,
                pair,
                ParseState::Name(span, required, doc, attributes),
                context,
            ),
        },

        ParseState::Name(span, required, doc, attributes) => {
            let name = GTKey::parse(pair);

            context.enter_parent(GTContextParent::Property(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    ParseState::Descriptor(span, required, doc, attributes, name),
                    context,
                ),
                None => Err(GTParseError::Internal(span, GTNode::Property)),
            }
        }

        ParseState::Descriptor(span, required, doc, attributes, name) => {
            let descriptor = GTDescriptor::parse(pair, context)?;
            Ok(GTProperty {
                span,
                doc,
                attributes,
                name,
                descriptor,
                required,
            })
        }
    }
}

enum ParseState {
    Doc(GTSpan, bool, Option<GTDoc>),
    Attributes(GTSpan, bool, Option<GTDoc>, Vec<GTAttribute>),
    Name(GTSpan, bool, Option<GTDoc>, Vec<GTAttribute>),
    Descriptor(GTSpan, bool, Option<GTDoc>, Vec<GTAttribute>, GTKey),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::required_property, "world: string").unwrap();
        assert_ron_snapshot!(
            GTProperty::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            @r#"
        GTProperty(
          span: GTSpan(0, 13),
          doc: None,
          attributes: [],
          name: GTKey(GTSpan(0, 5), "world"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(7, 13),
            kind: String,
            doc: None,
            attributes: [],
          )),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut pairs = GenotypeParser::parse(Rule::required_property, "world: string").unwrap();
        let parents = vec![GTContextParent::Alias(GTIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GTContext {
            module_id: "module".into(),
            parents: parents.clone(),
            resolve: GTModuleResolve::new(),
            claimed_names: Default::default(),
            annotation: None,
        };

        GTProperty::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
    }
}
