use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSAny {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "any"
        );
    }
}
