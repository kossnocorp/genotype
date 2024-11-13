use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

use super::{RSEnumVariantDescriptor, RSRender};

impl RSRender for RSEnumVariantDescriptor {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        match self {
            RSEnumVariantDescriptor::Descriptor(descriptor) => descriptor.render(indent, config),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_descriptor() {
        assert_eq!(
            RSEnumVariantDescriptor::Descriptor(RSDescriptor::Primitive(RSPrimitive::Boolean))
                .render(&rs_indent(), &Default::default()),
            "bool"
        );
        assert_eq!(
            RSEnumVariantDescriptor::Descriptor(RSDescriptor::Primitive(RSPrimitive::String))
                .render(&rs_indent(), &Default::default()),
            "String"
        );
    }
}
