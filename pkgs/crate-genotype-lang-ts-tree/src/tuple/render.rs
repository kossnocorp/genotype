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
        Ok(if context.is_zod_mode() {
            format!("z.tuple([{}])", descriptors)
        } else {
            format!("[{}]", descriptors)
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
}
