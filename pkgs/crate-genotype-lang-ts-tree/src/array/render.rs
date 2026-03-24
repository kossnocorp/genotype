use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSArray {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

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
            TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Array<string>"
        );
    }
}
