use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{options, PYOptions, PYRender};

use super::PYList;

impl PYRender for PYList {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        format!("list[{}]", self.descriptor.render(indent, options))
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYOptions::default()),
            "list[str]"
        );
    }
}
