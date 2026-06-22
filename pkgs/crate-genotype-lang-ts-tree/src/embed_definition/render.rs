use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsEmbedDefinition {
    fn render(
        &self,
        state: TsRenderState,
        _context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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
