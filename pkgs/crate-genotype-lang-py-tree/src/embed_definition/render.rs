use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyEmbedDefinition {
    fn render(
        &self,
        state: PyRenderState,
        _context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        Ok(self.embed.render(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyEmbedDefinition {
                name: "Name".into(),
                embed: r#"class Hello:\n    name = "World""#.into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"class Hello:\n    name = "World""#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            PyEmbedDefinition {
                name: "Name".into(),
                embed: r#"class Hello:\n    name = "World""#.into()
            }
            .render(
                PyRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"    class Hello:\n    name = "World""#
        );
    }
}
