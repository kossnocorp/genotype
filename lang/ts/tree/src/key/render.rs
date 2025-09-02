use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSKey {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSKey("foo".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "foo"
        );
    }
}
