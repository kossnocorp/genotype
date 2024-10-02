use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSArray;

impl GTRender for TSArray {
    fn render(&self, indent: &GTIndent) -> String {
        format!("Array<{}>", self.descriptor.render(indent))
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptor::TSDescriptor, indent::ts_indent, primitive::TSPrimitive};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&ts_indent()),
            "Array<string>"
        );
    }
}
