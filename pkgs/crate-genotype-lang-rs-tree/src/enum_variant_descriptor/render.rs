use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsEnumVariantDescriptor {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        Ok(match self {
            RsEnumVariantDescriptor::Descriptor(descriptor) => descriptor.render(state, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_descriptor() {
        assert_snapshot!(
            RsEnumVariantDescriptor::Descriptor(RsDescriptor::Primitive(RsPrimitive::Boolean))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            RsEnumVariantDescriptor::Descriptor(RsDescriptor::Primitive(RsPrimitive::String))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"String"
        );
    }
}
