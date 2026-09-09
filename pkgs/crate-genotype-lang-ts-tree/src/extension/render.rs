use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsExtension {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        match context.mode() {
            TsMode::Zod => self.reference.identifier.render(state, context),
            TsMode::Types | TsMode::Effect => self.reference.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[test]
    fn test_render_generic_by_mode() {
        let extension = TsExtension {
            reference: Tst::reference_with_arguments("Base", vec![Tst::primitive_string().into()]),
        };
        assert_snapshot!(
            render_node_with(extension.clone(), &mut Tst::render_context()),
            @"Base<string>"
        );
        assert_snapshot!(
            render_node_with(extension.clone(), &mut Tst::render_context_zod()),
            @"Base"
        );
        assert_snapshot!(
            render_node_with(extension, &mut Tst::render_context_effect()),
            @"Base(Schema.String)"
        );
    }
}
