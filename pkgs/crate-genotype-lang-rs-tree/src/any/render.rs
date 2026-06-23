use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsAny {
    fn render(
        &self,
        _state: RsRenderState,
        _context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        Ok("Any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsAny
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
