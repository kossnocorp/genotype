use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsExtension {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if context.is_zod_mode() {
            return self.reference.identifier.render(state, context);
        }

        self.reference.render(state, context)
    }
}

#[cfg(test)]
mod tests {

    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::extension("Foo")),
            @"Foo"
        );
    }

    #[test]
    fn test_render_zod() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::extension("Foo"), &mut context),
            @"Foo"
        );
    }
}
