use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsAttribute {
    fn render(
        &self,
        state: RsRenderState,
        _context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
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
