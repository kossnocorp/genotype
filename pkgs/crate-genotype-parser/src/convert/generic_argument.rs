use crate::prelude::internal::*;

impl GtParse for GtGenericArgument {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = token.span;
        let token = match token.kind {
            GtTokenKind::GenericArgument => {
                token
                    .into_semantic_children()
                    .next()
                    .ok_or(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::GenericArgument,
                        "generic argument inner",
                    ))?
            }

            _ => token,
        };
        let descriptor = GtDescriptor::parse(token, context)?;
        Ok(Self { span, descriptor })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtGenericArgument, to_parse_args(GtTokenKind::Descriptors, "string | number")),
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
