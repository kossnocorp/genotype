use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsAttribute {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(state.indent_format(&format!("#[{content}]", content = self.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsAttribute("derive".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"#[derive]"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RsAttribute("derive".into())
                .render(
                    RsRenderState::default().indent_inc(),
                    &mut Default::default()
                )
                .unwrap(),
            @"    #[derive]"
        );
    }
}
