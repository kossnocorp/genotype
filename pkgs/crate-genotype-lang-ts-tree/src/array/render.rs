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
        Ok(format!("Array<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            TsArray {
                descriptor: TsDescriptor::Primitive(TsPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Array<string>"
        );
    }
}
