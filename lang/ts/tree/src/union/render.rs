use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSUnion;

impl GTRender for TSUnion {
    fn render(&self, indent: &GTIndent) -> String {
        self.descriptors
            .iter()
            .map(|d| d.render(indent))
            .collect::<Vec<String>>()
            .join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor};

    #[test]
    fn test_render_union() {
        let indent = ts_indent();
        assert_eq!(
            TSUnion {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                    TSTypeDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(&indent),
            "string | number"
        );
    }
}
