use crate::prelude::internal::*;

impl GtParse for GtAttributeAssignment {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeAssignment,
            "attribute assignment inner",
        ))?;

        Ok(GtAttributeAssignment {
            span,
            value: GtAttributeValue::parse(token, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::AttributeAssignment, "= 42").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeAssignment::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @"
        GtAttributeAssignment(
          span: GtSpan(0, 4),
          value: Literal(GtLiteral(
            span: GtSpan(2, 4),
            doc: None,
            attributes: [],
            value: Integer(42),
          )),
        )
        "
        );
    }
}
