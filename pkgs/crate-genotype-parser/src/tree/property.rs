use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtProperty {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtKey,
    #[visit]
    pub descriptor: GtDescriptor,
    pub required: bool,
}

impl GtProperty {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let required = pair.as_rule() == Rule::required_property;
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GtParseError::Internal(span, GtNode::Property))?;
        let property = parse(inner, pair, ParseState::Doc(span, required, None), context)?;

        context.exit_parent(span, GtNode::Property)?;

        Ok(property)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtProperty> {
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
                    None => Err(GtParseError::Internal(span, GtNode::Property)),
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
                let attribute = GtAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Attributes(span, required, doc, attributes),
                        context,
                    ),
                    None => Err(GtParseError::Internal(span, GtNode::Property)),
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
            let name = GtKey::parse(pair);

            context.enter_parent(GtContextParent::Property(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    ParseState::Descriptor(span, required, doc, attributes, name),
                    context,
                ),
                None => Err(GtParseError::Internal(span, GtNode::Property)),
            }
        }

        ParseState::Descriptor(span, required, doc, attributes, name) => {
            let descriptor = GtDescriptor::parse(pair, context)?;
            Ok(GtProperty {
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
    Doc(GtSpan, bool, Option<GtDoc>),
    Attributes(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>),
    Name(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>),
    Descriptor(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>, GtKey),
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
            GtProperty::parse(pairs.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            @r#"
        GtProperty(
          span: GtSpan(0, 13),
          doc: None,
          attributes: [],
          name: GtKey(GtSpan(0, 5), "world"),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(7, 13),
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
        let parents = vec![GtContextParent::Alias(GtIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GtContext {
            module_id: "module".into(),
            parents: parents.clone(),
            resolve: GtModuleResolve::new(),
            claimed_names: Default::default(),
            annotation: None,
        };

        GtProperty::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
    }
}
