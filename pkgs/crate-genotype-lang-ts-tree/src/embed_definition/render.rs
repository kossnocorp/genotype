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
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(
                Tst::embed_definition(
                    "Name",
                    r#"const hello = {
  name: "World"
};"#,
                ),
            ),
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
            Tst::embed_definition(
                "Name",
                r#"const hello = {
  name: "World"
};"#,
            )
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
