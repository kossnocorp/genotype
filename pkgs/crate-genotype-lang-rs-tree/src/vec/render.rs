use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsVec {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("Vec<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            RsVec {
                descriptor: RsDescriptor::Primitive(RsPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Vec<String>"
        );
    }
}
