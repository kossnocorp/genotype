use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtAttributeDescriptor {
    Assignment(#[visit] GtAttributeAssignment),
    Arguments(Vec<GtAttributeValue>),
    Properties(#[visit] Vec<GtAttributeProperty>),
}

impl From<GtAttributeAssignment> for GtAttributeDescriptor {
    fn from(value: GtAttributeAssignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<Vec<GtAttributeValue>> for GtAttributeDescriptor {
    fn from(value: Vec<GtAttributeValue>) -> Self {
        Self::Arguments(value)
    }
}

impl From<Vec<GtAttributeProperty>> for GtAttributeDescriptor {
    fn from(value: Vec<GtAttributeProperty>) -> Self {
        Self::Properties(value)
    }
}

impl GtAttributeDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or(GtParseError::UnexpectedEnd(span, GtNode::AttributeDescriptor))?;

        match pair.as_rule() {
            Rule::attribute_assignment => Ok(GtAttributeDescriptor::Assignment(
                GtAttributeAssignment::parse(pair, context)?,
            )),

            Rule::attribute_arguments => {
                let arguments = pair
                    .into_inner()
                    .map(|pair| GtAttributeValue::parse(pair, context))
                    .collect::<Result<_, _>>()?;
                Ok(GtAttributeDescriptor::Arguments(arguments))
            }

            Rule::attribute_properties => {
                let properties = pair
                    .into_inner()
                    .map(|rule| GtAttributeProperty::parse(rule, context))
                    .collect::<Result<_, _>>()?;
                Ok(GtAttributeDescriptor::Properties(properties))
            }

            rule => Err(GtParseError::UnexpectedRule(
                span,
                GtNode::AttributeDescriptor,
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
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        Assignment(GtAttributeAssignment(
          span: GtSpan(0, 4),
          value: Literal(GtLiteral(
            span: GtSpan(2, 4),
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
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        Arguments([
          Literal(GtLiteral(
            span: GtSpan(1, 8),
            doc: None,
            attributes: [],
            value: String("hello"),
          )),
          Literal(GtLiteral(
            span: GtSpan(10, 17),
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
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        Properties([
          GtAttributeProperty(
            span: GtSpan(1, 16),
            name: GtAttributeKey(
              span: GtSpan(1, 6),
              value: "hello",
            ),
            value: Literal(GtLiteral(
              span: GtSpan(9, 16),
              doc: None,
              attributes: [],
              value: String("world"),
            )),
          ),
          GtAttributeProperty(
            span: GtSpan(18, 27),
            name: GtAttributeKey(
              span: GtSpan(18, 21),
              value: "qwe",
            ),
            value: Literal(GtLiteral(
              span: GtSpan(24, 27),
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
