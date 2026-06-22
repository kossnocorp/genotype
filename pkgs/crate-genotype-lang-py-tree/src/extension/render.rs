use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyExtension {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        self.reference.render(state, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyExtension {
                reference: PyReference::new("Foo".into(), false)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Foo"
        );
    }
}
