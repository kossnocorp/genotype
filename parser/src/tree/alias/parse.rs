use crate::prelude::internal::*;

impl GTAlias {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Alias))?;
        let alias = parse(inner, pair, context, ParseState::Doc(span.clone(), None))?;

        context.exit_parent(span, GTNode::Alias)?;

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
                    None => Err(GTParseError::Internal(span, GTNode::Alias)),
                }
            }

            _ => parse(
                inner,
                pair,
                context,
                ParseState::Attributes(span, doc_acc, vec![]),
            ),
        },

        ParseState::Attributes(span, doc, attributes) => match pair.as_rule() {
            Rule::attribute => {
                let attribute = GTAttribute::parse(pair, context)?;
                let mut attributes = attributes;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Attributes(span, doc, attributes),
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Alias)),
                }
            }

            _ => parse(
                inner,
                pair,
                context,
                ParseState::Name(span, doc, attributes),
            ),
        },

        ParseState::Name(span, doc, attributes) => {
            let name: GTIdentifier = pair.into();

            context.resolve.exports.push(name.clone());
            context.enter_parent(GTContextParent::Alias(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, doc, attributes, name),
                ),
                None => Err(GTParseError::Internal(span, GTNode::Alias)),
            }
        }

        ParseState::Descriptor(span, doc, attributes, name) => {
            let id = context.module_id.definition_id(&name);
            let descriptor = GTDescriptor::parse(pair, context)?;
            Ok(GTAlias {
                id,
                span,
                doc,
                attributes,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Doc(GTSpan, Option<GTDoc>),
    Attributes(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
    Name(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
    Descriptor(GTSpan, Option<GTDoc>, Vec<GTAttribute>, GTIdentifier),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello: { world: string }").unwrap();
        assert_ron_snapshot!(
            GTAlias::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            @r#"
        GTAlias(
          id: GTDefinitionId(GTModuleId("module"), "Hello"),
          span: GTSpan(0, 24),
          doc: None,
          attributes: [],
          name: GTIdentifier(GTSpan(0, 5), "Hello"),
          descriptor: Object(GTObject(
            span: GTSpan(7, 24),
            name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
            extensions: [],
            properties: [
              GTProperty(
                span: GTSpan(9, 22),
                doc: None,
                attributes: [],
                name: GTKey(GTSpan(9, 14), "world"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(16, 22),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
                required: true,
              ),
            ],
          )),
        )
        "#
        );
    }

    #[test]
    fn test_parse_exports() {
        let source_code = NamedSource::new("module.type", "Hello: string".into());
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.exports,
            @r#"
        [
          GTIdentifier(GTSpan(0, 5), "Hello"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello: { world: string }").unwrap();
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

        GTAlias::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
    }
}
