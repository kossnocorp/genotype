use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsAny {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok("any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"any"
        );
    }
}
