use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsReference {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let reference = self.identifier.render(state, context)?;

        if self.arguments.is_empty() {
            return Ok(reference);
        }

        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.render(state, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");

        match context.mode() {
            TsMode::Zod | TsMode::Effect => Ok(format!("{reference}({arguments})")),
            TsMode::Types => Ok(format!("{reference}<{arguments}>")),
        }
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
    fn test_render_with_arguments() {
        assert_snapshot!(
            render_node(Tst::reference_with_arguments("Foo", vec![Tst::primitive_string().into()])),
            @"Foo<string>"
        );

        assert_snapshot!(
            render_node(Tst::reference_with_arguments("Foo", vec![Tst::primitive_string().into(), Tst::primitive_number().into()])),
            @"Foo<string, number>"
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
        assert_snapshot!(
            render_node_with(Tst::reference("Bar"), &mut Tst::render_context_zod()),
            @"Bar"
        );
    }

    #[test]
    fn test_render_zod_forward() {
        assert_snapshot!(
            render_node_with(Tst::reference_forward("Bar"), &mut Tst::render_context_zod()),
            @"Bar"
        );
    }

    #[test]
    fn test_render_zod_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::reference_with_arguments("Bar", vec![Tst::primitive_string().into()]),
                &mut Tst::render_context_zod(),
            ),
            @"Bar(z.string())"
        );
    }

    #[test]
    fn test_render_effect() {
        assert_snapshot!(
            render_node_with(Tst::reference("Bar"), &mut Tst::render_context_effect()),
            @"Bar"
        );
    }

    #[test]
    fn test_render_effect_forward() {
        assert_snapshot!(
            render_node_with(Tst::reference_forward("Bar"), &mut Tst::render_context_effect()),
            @"Bar"
        );
    }

    #[test]
    fn test_render_effect_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::reference_with_arguments("Bar", vec![Tst::primitive_string().into()]),
                &mut Tst::render_context_effect(),
            ),
            @"Bar(Schema.String)"
        );
    }
}
