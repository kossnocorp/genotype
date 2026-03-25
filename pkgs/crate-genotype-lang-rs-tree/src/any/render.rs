use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsAny {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

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
    fn test_render() {
        assert_snapshot!(
            RsAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
