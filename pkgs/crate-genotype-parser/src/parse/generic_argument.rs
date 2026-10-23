use crate::prelude::internal::*;

impl GtParse for GtGenericArgument {
    fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = (pair.as_span().start(), pair.as_span().end()).into();
        let pair = match pair.as_rule() {
            Rule::generic_argument => {
                pair.into_inner().next().ok_or(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::GenericArgument,
                    "generic argument inner",
                ))?
            }

            _ => pair,
        };
        let descriptor = GtDescriptor::parse(pair, context)?;
        Ok(Self { span, descriptor })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtGenericArgument, to_parse_args(Rule::descriptors, "string | number")),
            @"
        GtGenericArgument(
          span: GtSpan(0, 15),
          descriptor: Union(GtUnion(
            span: GtSpan(0, 15),
            doc: None,
            attributes: [],
            descriptors: [
              Primitive(GtPrimitive(
                span: GtSpan(0, 6),
                kind: String,
                doc: None,
                attributes: [],
              )),
              Primitive(GtPrimitive(
                span: GtSpan(9, 15),
                kind: Number,
                doc: None,
                attributes: [],
              )),
            ],
          )),
        )
        "
        );
    }
}
