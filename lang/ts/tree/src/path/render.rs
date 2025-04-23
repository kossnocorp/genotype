use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSPath {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

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
            TSPath("./path/to/module.ts".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "./path/to/module.ts"
        );
    }
}
