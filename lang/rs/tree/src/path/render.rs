use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSPath {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSPath(
                GTModuleId("path/to/module".into()),
                "self::path::to::module".into()
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "self::path::to::module"
        );
    }
}
