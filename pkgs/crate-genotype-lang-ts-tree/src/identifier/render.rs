use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsIdentifier {
    fn render(
        &self,
        _state: TsRenderState,
        _context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        Ok(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {

    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::identifier("Foo")),
            @"Foo"
        );
    }
}
