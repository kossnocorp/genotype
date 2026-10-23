use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsTuple {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(match context.mode() {
            TsMode::Effect => format!("Schema.mutable(Schema.Tuple([{descriptors}]))"),
            TsMode::Zod => format!("z.tuple([{descriptors}])"),
            TsMode::Types => format!("[{descriptors}]"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            render_node(
                Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
            ),
            @"[string, number]"
        );
    }

    #[test]
    fn test_render_tuple_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
                &mut context,
            ),
            @"z.tuple([z.string(), z.number()])"
        );
    }

    #[test]
    fn test_render_effect_tuple() {
        assert_snapshot!(
            render_node_with(Tst::tuple(vec_into![Tst::primitive_string(), Tst::primitive_number()]), &mut Tst::render_context_effect()),
            @r#"Schema.mutable(Schema.Tuple([Schema.String, Schema.Number]))"#
        );
    }

    #[test]
    fn test_render_effect_empty() {
        assert_snapshot!(
            render_node_with(Tst::tuple(vec![]), &mut Tst::render_context_effect()),
            @r#"Schema.mutable(Schema.Tuple([]))"#
        );
    }
}
