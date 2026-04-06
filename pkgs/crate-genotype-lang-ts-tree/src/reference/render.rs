use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsReference {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let reference = self.identifier.render(state, context)?;
        Ok(reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::reference("Foo")),
            @"Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_snapshot!(
            render_node(Tst::reference_forward("Foo")),
            @"Foo"
        );
    }

    #[test]
    fn test_render_zod() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::reference("Bar"), &mut context),
            @"Bar"
        );
    }

    #[test]
    fn test_render_zod_forward() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::reference_forward("Bar"), &mut context),
            @"Bar"
        );
    }
}
