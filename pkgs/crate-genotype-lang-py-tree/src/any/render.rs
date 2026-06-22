use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyAny {

    fn render(
        &self,
        _state: PyRenderState,
        _context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        Ok("Any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            PyAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
