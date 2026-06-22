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

        if context.is_zod_mode() {
            Ok(format!("z.union([{}])", descriptors.join(", ")))
        } else {
            Ok(descriptors.join(" | "))
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
}
