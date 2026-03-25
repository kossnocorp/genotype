use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsPrimitive {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TsPrimitive::String => "string",
            TsPrimitive::Number => "number",
            TsPrimitive::Boolean => "boolean",
            TsPrimitive::BigInt => "bigint",
            TsPrimitive::Null => "null",
            TsPrimitive::Undefined => "undefined",
        }
        .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            TsPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"string"
        );
        assert_snapshot!(
            TsPrimitive::Number
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"number"
        );
        assert_snapshot!(
            TsPrimitive::BigInt
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bigint"
        );
        assert_snapshot!(
            TsPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"boolean"
        );
        assert_snapshot!(
            TsPrimitive::Null
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"null"
        );
        assert_snapshot!(
            TsPrimitive::Undefined
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"undefined"
        );
    }
}
