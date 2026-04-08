use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsTuple {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
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
