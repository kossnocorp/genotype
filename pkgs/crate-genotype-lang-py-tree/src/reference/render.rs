use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyReference {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
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
                        ..Default::default()
                    }
                )
                .unwrap(),
            @r#""Foo""#
        );
    }
}
