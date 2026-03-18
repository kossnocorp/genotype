use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSPrimitive {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TSPrimitive::String => "string",
            TSPrimitive::Number => "number",
            TSPrimitive::Boolean => "boolean",
            TSPrimitive::BigInt => "bigint",
            TSPrimitive::Null => "null",
            TSPrimitive::Undefined => "undefined",
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
            TSPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"string"
        );
        assert_snapshot!(
            TSPrimitive::Number
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"number"
        );
        assert_snapshot!(
            TSPrimitive::BigInt
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bigint"
        );
        assert_snapshot!(
            TSPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"boolean"
        );
        assert_snapshot!(
            TSPrimitive::Null
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"null"
        );
        assert_snapshot!(
            TSPrimitive::Undefined
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"undefined"
        );
    }
}
