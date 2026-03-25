use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsPath {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(self.1.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsPath(
                GtModuleId("path/to/module".into()),
                "self::path::to::module".into()
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"self::path::to::module"
        );
    }
}
