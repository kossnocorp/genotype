use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsPath {
    fn render(
        &self,
        _state: RsRenderState,
        _context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
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
