use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSEmbedDefinition {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

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
            TSEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"const hello = {
  name: "World"
};"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            }
            .render(
                TSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"  const hello = {
    name: "World"
  };"#
        );
    }
}
