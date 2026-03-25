use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsOption {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
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
