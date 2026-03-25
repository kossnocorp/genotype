use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsEnumVariantDescriptor {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
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
