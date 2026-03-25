use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyEmbedDefinition {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
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
