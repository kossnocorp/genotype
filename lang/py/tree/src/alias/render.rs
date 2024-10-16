use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender};

use super::PYAlias;

impl PYRender for PYAlias {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        format!(
            "type {} = {};",
            self.name.render(indent),
            self.descriptor.render(indent, options)
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYOptions::default()),
            "type Name = str;"
        );
        assert_eq!(
            PYDescriptor::Primitive(PYPrimitive::String)
                .render(&py_indent(), &PYOptions::default()),
            "str"
        );
    }
}
