use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsIdentifier {
    fn render(
        &self,
        _state: RsRenderState,
        _context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        Ok(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsIdentifier("Foo".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Foo"
        );
    }
}
