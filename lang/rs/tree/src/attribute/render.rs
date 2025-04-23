use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSAttribute {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "#[derive]"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(
                    RSRenderState::default().indent_inc(),
                    &mut Default::default()
                )
                .unwrap(),
            "    #[derive]"
        );
    }
}
