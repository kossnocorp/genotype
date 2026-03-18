use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYPath {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

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
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PYPath(".path.to.module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @".path.to.module"
        );
    }
}
