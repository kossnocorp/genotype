use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsArray {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(if context.is_zod_mode() {
            format!("z.array({descriptor})")
        } else {
            format!("Array<{descriptor}>")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
