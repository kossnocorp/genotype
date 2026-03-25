use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyDoc {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        let lines = self.0.split("\n").enumerate();
        Ok(lines
            .map(|(index, line)| {
                let comment = if index == 0 { r#"""""# } else { "" };
                state.indent_format(&format!("{comment}{line}"))
            })
            .collect::<Vec<_>>()
            .join("\n")
            + r#"""""#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_simple() {
        assert_snapshot!(
            PyDoc("Hello, world!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#""""Hello, world!""""#
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_snapshot!(
            PyDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        """Hello,
        cruel
        world!"""
        "#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            PyDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(
                PyRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"
        """Hello,
        cruel
        world!"""
        "#
        );
    }
}
