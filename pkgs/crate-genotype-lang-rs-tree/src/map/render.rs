use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsMap {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
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
            RsMap {
                key: RsPrimitive::String.into(),
                descriptor: RsPrimitive::IntSize.into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"BTreeMap<String, isize>"
        );
    }
}
