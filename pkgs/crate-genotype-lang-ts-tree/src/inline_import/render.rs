use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInlineImport {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;

        if context.is_zod_mode() {
            return Ok(name);
        }

        let path = self.path.render(state, context)?;

        Ok(format!(r#"import("{path}").{name}"#))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::inline_import("./path/to/module", "Name")),
            @r#"import("./path/to/module.js").Name"#
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::inline_import("./path/to/module", "Name"), &mut context),
            @"Name"
        );
    }
}
