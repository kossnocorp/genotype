use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsVec {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
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
