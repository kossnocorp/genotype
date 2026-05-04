use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtGenericArgument {
    pub span: GtSpan,
    #[visit]
    pub descriptor: GtDescriptor,
}

impl GtGenericArgument {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let descriptor = GtDescriptor::parse(pair, context)?;
        Ok(Self { span, descriptor })
    }
}

impl<Type: Into<GtDescriptor>> From<Type> for GtGenericArgument {
    fn from(descriptor: Type) -> Self {
        let descriptor = descriptor.into();
        GtGenericArgument {
            span: descriptor.span(),
            descriptor,
        }
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
