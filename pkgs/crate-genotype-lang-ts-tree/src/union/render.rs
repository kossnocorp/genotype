use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsUnion {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>, _>>()?;

        match context.mode() {
            TsMode::Effect => Ok(format!("Schema.Union([{}])", descriptors.join(", "))),
            TsMode::Zod => Ok(format!("z.union([{}])", descriptors.join(", "))),
            TsMode::Types => Ok(descriptors.join(" | ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            render_node(
                Tst::union(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
            ),
            @"string | number"
        );
    }

    #[test]
    fn test_render_union_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::union(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
                &mut context,
            ),
            @"z.union([z.string(), z.number()])"
        );
    }

    #[test]
    fn test_render_union_effect_mode() {
        let mut context = Tst::render_context_effect();

        assert_snapshot!(
            render_node_with(
                Tst::union(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
                &mut context,
            ),
            @"Schema.Union([Schema.String, Schema.Number])"
        );
    }
}
