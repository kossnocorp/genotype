use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSMap {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("BTreeMap<{key}, {descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RSMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::IntSize.into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"BTreeMap<String, isize>"
        );
    }
}
