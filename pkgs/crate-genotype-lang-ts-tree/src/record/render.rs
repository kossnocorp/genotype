use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsRecord {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        Ok(if context.is_zod_mode() {
            format!("z.record({key}, {descriptor})")
        } else {
            format!("Record<{key}, {descriptor}>")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(
                Tst::record(Tst::record_key_number(), Tst::primitive_string()),
            ),
            @"Record<number, string>"
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::record(Tst::record_key_number(), Tst::primitive_string()),
                &mut context,
            ),
            @"z.record(z.number(), z.string())"
        );
    }
}
