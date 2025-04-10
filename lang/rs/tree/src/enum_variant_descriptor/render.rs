use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSEnumVariantDescriptor {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            RSEnumVariantDescriptor::Descriptor(descriptor) => descriptor.render(context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_descriptor() {
        assert_eq!(
            RSEnumVariantDescriptor::Descriptor(RSDescriptor::Primitive(RSPrimitive::Boolean))
                .render(&mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSEnumVariantDescriptor::Descriptor(RSDescriptor::Primitive(RSPrimitive::String))
                .render(&mut Default::default())
                .unwrap(),
            "String"
        );
    }
}
