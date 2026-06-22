use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyPath {
    fn render(
        &self,
        _state: PyRenderState,
        _context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        Ok(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyPath(".path.to.module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @".path.to.module"
        );
    }
}
