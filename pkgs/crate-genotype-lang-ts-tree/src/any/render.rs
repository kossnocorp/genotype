use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsAny {
    fn render(
        &self,
        _state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        Ok(if context.is_zod_mode() {
            "z.any()".into()
        } else {
            "any".into()
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::any()),
            @"any"
        );
    }

    #[test]
    fn test_render_zod() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::any(), &mut context),
            @"z.any()"
        );
    }
}
