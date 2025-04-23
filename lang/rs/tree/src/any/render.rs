use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSAny {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            "Any",
            RSAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
        );
    }
}
