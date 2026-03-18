use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSOption {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

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
            RSOption {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(RSRenderState::default(), &mut Default::default())
            .unwrap(),
            @"Option<String>"
        );
    }
}
