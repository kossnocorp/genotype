use genotype_lang_core::{indent::GTIndent, render::GTRender};

use super::TSArray;

impl GTRender for TSArray {
    fn render(&self, indent: &GTIndent) -> String {
        format!("Array<{}>", self.descriptor.render(indent))
    }
}

#[cfg(test)]
mod tests {
    use crate::{indent::ts_indent, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor};

    use super::*;

    #[test]
    fn test_render_array() {
        let indent = ts_indent();
        assert_eq!(
            TSArray {
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&indent),
            "Array<string>"
        );
    }
}
