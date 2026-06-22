use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsFieldName {

    fn render(
        &self,
        _state: RsRenderState,
        _context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        Ok(RsNaming::render(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsFieldName("foo".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"foo"
        );
    }

    #[test]
    fn test_render_keyword() {
        assert_snapshot!(
            RsFieldName("type".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"r#type"
        );
    }
}
