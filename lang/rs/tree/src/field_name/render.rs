use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSFieldName {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(RSNaming::render(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSFieldName("foo".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "foo"
        );
    }

    #[test]
    fn test_render_keyword() {
        assert_eq!(
            RSFieldName("type".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "r#type"
        );
    }
}
