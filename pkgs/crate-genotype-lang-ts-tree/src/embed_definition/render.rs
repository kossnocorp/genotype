use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsEmbedDefinition {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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
            TsEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        const hello = {
          name: "World"
        };
        "#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            }
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"
        const hello = {
          name: "World"
        };
        "#
        );
    }
}
