use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsArray {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(match context.mode() {
            TsMode::Effect => format!("Schema.mutable(Schema.Array({descriptor}))"),
            TsMode::Zod => format!("z.array({descriptor})"),
            TsMode::Types => format!("Array<{descriptor}>"),
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            render_node(Tst::array(Tst::primitive_string())),
            @"Array<string>"
        );
    }

    #[test]
    fn test_render_array_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::array(Tst::primitive_string()), &mut context),
            @"z.array(z.string())"
        );
    }

    #[test]
    fn test_render_effect_array() {
        assert_snapshot!(
            render_node_with(Tst::array(Tst::primitive_string()), &mut Tst::render_context_effect()),
            @r#"Schema.mutable(Schema.Array(Schema.String))"#
        );
    }
}
