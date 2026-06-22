use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyReference {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let str = self.identifier.render(state, context)?;
        if let PyVersion::Legacy = context.config.version
            && self.forward
        {
            return Ok(format!("\"{str}\""));
        }
        Ok(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyReference::new("Foo".into(), false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_snapshot!(
            PyReference::new("Foo".into(), true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Foo"
        );
        assert_snapshot!(
            PyReference::new("Foo".into(), true)
                .render(
                    Default::default(),
                    &mut PyRenderContext {
                        config: &PyConfigLang::new(PyVersion::Legacy),
                    }
                )
                .unwrap(),
            @r#""Foo""#
        );
    }
}
