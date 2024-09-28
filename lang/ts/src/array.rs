use genotype_lang_core::{indent::Indent, node::Node};

use crate::type_descriptor::TSTypeDescriptor;

pub struct TSArray {
    pub descriptor: TSTypeDescriptor,
}

impl Node for TSArray {
    fn render(&self, indent: &Indent) -> String {
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
