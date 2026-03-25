use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GTAttributeDescriptor {
    Assignment(#[visit] GTAttributeAssignment),
    Arguments(Vec<GTAttributeValue>),
    Properties(#[visit] Vec<GTAttributeProperty>),
}

impl From<GTAttributeAssignment> for GTAttributeDescriptor {
    fn from(value: GTAttributeAssignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<Vec<GTAttributeValue>> for GTAttributeDescriptor {
    fn from(value: Vec<GTAttributeValue>) -> Self {
        Self::Arguments(value)
    }
}

impl From<Vec<GTAttributeProperty>> for GTAttributeDescriptor {
    fn from(value: Vec<GTAttributeProperty>) -> Self {
        Self::Properties(value)
    }
}

impl GTAttributeDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(|| {
            GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeDescriptor)
        })?;

        match pair.as_rule() {
            Rule::attribute_assignment => Ok(GTAttributeDescriptor::Assignment(
                GTAttributeAssignment::parse(pair, context)?,
            )),

            Rule::attribute_arguments => {
                let arguments = pair
                    .into_inner()
                    .map(|pair| GTAttributeValue::parse(pair, context))
                    .collect::<Result<_, _>>()?;
                Ok(GTAttributeDescriptor::Arguments(arguments))
            }

            Rule::attribute_properties => {
                let properties = pair
                    .into_inner()
                    .map(|rule| GTAttributeProperty::parse(rule, context))
                    .collect::<Result<_, _>>()?;
                Ok(GTAttributeDescriptor::Properties(properties))
            }

            rule => Err(GTParseError::UnexpectedRule(
                span,
                GTNode::AttributeDescriptor,
                rule,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;

    #[test]
    fn test_parse_assignment() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_descriptor, "= 42").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        Assignment(GTAttributeAssignment(
          span: GTSpan(0, 4),
          value: Literal(GTLiteral(
            span: GTSpan(2, 4),
            doc: None,
            attributes: [],
            value: Integer(42),
          )),
        ))
        "
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute_descriptor, r#"("hello", "world")"#).unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        Arguments([
          Literal(GTLiteral(
            span: GTSpan(1, 8),
            doc: None,
            attributes: [],
            value: String("hello"),
          )),
          Literal(GTLiteral(
            span: GTSpan(10, 17),
            doc: None,
            attributes: [],
            value: String("world"),
          )),
        ])
        "#
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut pairs = GenotypeParser::parse(
            Rule::attribute_descriptor,
            r#"(hello = "world", qwe = 123)"#,
        )
        .unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        Properties([
          GTAttributeProperty(
            span: GTSpan(1, 16),
            name: GTAttributeKey(
              span: GTSpan(1, 6),
              value: "hello",
            ),
            value: Literal(GTLiteral(
              span: GTSpan(9, 16),
              doc: None,
              attributes: [],
              value: String("world"),
            )),
          ),
          GTAttributeProperty(
            span: GTSpan(18, 27),
            name: GTAttributeKey(
              span: GTSpan(18, 21),
              value: "qwe",
            ),
            value: Literal(GTLiteral(
              span: GTSpan(24, 27),
              doc: None,
              attributes: [],
              value: Integer(123),
            )),
          ),
        ])
        "#
        );
    }
}
