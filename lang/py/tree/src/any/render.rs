use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYAny {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok("Any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            PYAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
