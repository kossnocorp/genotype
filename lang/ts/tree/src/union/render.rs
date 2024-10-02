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
    use crate::{descriptor::TSDescriptor, indent::ts_indent, primitive::TSPrimitive};

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(&ts_indent()),
            "string | number"
        );
    }
}
