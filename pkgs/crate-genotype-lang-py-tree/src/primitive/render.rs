use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyPrimitive {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(&self, _state: PyRenderState, _context: &mut PyRenderContext) -> Result<String> {
        Ok(match self {
            PyPrimitive::Boolean => "bool",
            PyPrimitive::String => "str",
            PyPrimitive::Int => "int",
            PyPrimitive::Float => "float",
            PyPrimitive::None => "None",
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
            PyPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PyPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
        assert_snapshot!(
            PyPrimitive::Int
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"int"
        );
        assert_snapshot!(
            PyPrimitive::Float
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"float"
        );
        assert_snapshot!(
            PyPrimitive::None
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"None"
        );
    }
}
