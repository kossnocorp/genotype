use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsOption {

    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("Option<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsOption {
                descriptor: RsDescriptor::Primitive(RsPrimitive::String)
            }
            .render(RsRenderState::default(), &mut Default::default())
            .unwrap(),
            @"Option<String>"
        );
    }
}
