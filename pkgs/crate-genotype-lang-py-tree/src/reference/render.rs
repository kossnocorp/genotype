use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyReference {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let mut str = self.identifier.render(state, context)?;
        if !self.arguments.is_empty() {
            let arguments = self
                .arguments
                .iter()
                .map(|argument| argument.render(state, context))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
            str = format!("{str}[{arguments}]");
        }
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

    #[test]
    fn test_render_with_arguments() {
        assert_snapshot!(
            PyReference::new_with_arguments(
                "Foo".into(),
                vec![PyDescriptor::Primitive(PyPrimitive::String)],
                false
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Foo[str]"
        );
    }

    #[test]
    fn test_render_forward_with_arguments() {
        assert_snapshot!(
            PyReference::new_with_arguments(
                "Foo".into(),
                vec![PyDescriptor::Primitive(PyPrimitive::String)],
                true
            )
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @r#""Foo[str]""#
        );
    }
}
