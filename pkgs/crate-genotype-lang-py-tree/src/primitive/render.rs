use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYPrimitive {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _state: PYRenderState, _context: &mut PYRenderContext) -> Result<String> {
        Ok(match self {
            PYPrimitive::Boolean => "bool",
            PYPrimitive::String => "str",
            PYPrimitive::Int => "int",
            PYPrimitive::Float => "float",
            PYPrimitive::None => "None",
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
            PYPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PYPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
        assert_snapshot!(
            PYPrimitive::Int
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"int"
        );
        assert_snapshot!(
            PYPrimitive::Float
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"float"
        );
        assert_snapshot!(
            PYPrimitive::None
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"None"
        );
    }
}
