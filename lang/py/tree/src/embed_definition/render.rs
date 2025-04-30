use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYEmbedDefinition {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYEmbedDefinition {
                name: "Name".into(),
                embed: r#"class Hello:
    name = "World""#
                    .into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Hello:
    name = "World""#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYEmbedDefinition {
                name: "Name".into(),
                embed: r#"class Hello:
    name = "World""#
                    .into()
            }
            .render(
                PYRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    class Hello:
        name = "World""#
        );
    }
}
