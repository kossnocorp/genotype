use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttribute {
    pub span: GtSpan,
    #[visit]
    pub name: GtAttributeName,
    #[visit]
    pub descriptor: Option<GtAttributeDescriptor>,
}

impl GtAttribute {
    pub fn new(
        span: GtSpan,
        name: GtAttributeName,
        descriptor: Option<GtAttributeDescriptor>,
    ) -> Self {
        Self {
            span,
            name,
            descriptor,
        }
    }

    pub fn is_it(&self, name: &str) -> bool {
        self.name.value.as_ref() == name
    }

    pub fn get_assigned(&self, name: &str) -> Option<&GtAttributeAssignment> {
        if self.is_it(name)
            && let Some(GtAttributeDescriptor::Assignment(assignment)) = &self.descriptor
        {
            return Some(assignment);
        }
        None
    }

    pub fn find_property(&self, name: &str) -> Option<String> {
        match &self.descriptor {
            Some(GtAttributeDescriptor::Assignment(assignment)) => {
                if self.name.value.as_ref() == name
                    && let GtAttributeValue::Literal(literal) = &assignment.value
                    && let GtLiteralValue::String(string) = &literal.value
                {
                    return Some(string.clone());
                }
            }
            Some(GtAttributeDescriptor::Properties(properties)) => {
                for property in properties.iter() {
                    if property.name.value.as_ref() == name
                        && let GtAttributeValue::Literal(literal) = &property.value
                        && let GtLiteralValue::String(string) = &literal.value
                    {
                        return Some(string.clone());
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub fn find_property_in(attributes: &Vec<GtAttribute>, name: &str) -> Option<String> {
        for attr in attributes.iter() {
            if let Some(value) = attr.find_property(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn find_flag(attributes: &Vec<GtAttribute>, name: &str) -> bool {
        attributes
            .iter()
            .any(|attr| attr.is_it(name) && attr.descriptor.is_none())
    }
}

impl GtAttribute {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or(GtParseError::UnexpectedEnd(span, GtNode::Attribute))?;

        parse(inner, pair, ParseState::Name(span), context)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtAttribute> {
    match state {
        ParseState::Name(span) => {
            let name_span: GtSpan = pair.as_span().into();
            let name = GtAttributeName::new(name_span, pair.as_str().into());

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Descriptor(span, name), context),

                None => Ok(GtAttribute {
                    span,
                    name,
                    descriptor: None,
                }),
            }
        }

        ParseState::Descriptor(span, name) => {
            let descriptor = match pair.as_rule() {
                Rule::attribute_descriptor => Some(GtAttributeDescriptor::parse(pair, context)?),
                _ => None,
            };

            Ok(GtAttribute {
                span,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Name(GtSpan),
    Descriptor(GtSpan, GtAttributeName),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;
    

    #[test]
    fn test_parse_simple() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[tag]").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 6),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "tag",
          ),
          descriptor: None,
        )
        "#
        );
    }

    #[test]
    fn test_parse_assignment() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[answer = 42]").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 14),
          name: GtAttributeName(
            span: GtSpan(2, 8),
            value: "answer",
          ),
          descriptor: Some(Assignment(GtAttributeAssignment(
            span: GtSpan(9, 13),
            value: Literal(GtLiteral(
              span: GtSpan(11, 13),
              doc: None,
              attributes: [],
              value: Integer(42),
            )),
          ))),
        )
        "#
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say("hello", "world")]"#).unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 24),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Arguments([
            Literal(GtLiteral(
              span: GtSpan(6, 13),
              doc: None,
              attributes: [],
              value: String("hello"),
            )),
            Literal(GtLiteral(
              span: GtSpan(15, 22),
              doc: None,
              attributes: [],
              value: String("world"),
            )),
          ])),
        )
        "#
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say(hello = "world", qwe = 123)]"#)
                .unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 34),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Properties([
            GtAttributeProperty(
              span: GtSpan(6, 21),
              name: GtAttributeKey(
                span: GtSpan(6, 11),
                value: "hello",
              ),
              value: Literal(GtLiteral(
                span: GtSpan(14, 21),
                doc: None,
                attributes: [],
                value: String("world"),
              )),
            ),
            GtAttributeProperty(
              span: GtSpan(23, 32),
              name: GtAttributeKey(
                span: GtSpan(23, 26),
                value: "qwe",
              ),
              value: Literal(GtLiteral(
                span: GtSpan(29, 32),
                doc: None,
                attributes: [],
                value: Integer(123),
              )),
            ),
          ])),
        )
        "#
        );
    }
}
