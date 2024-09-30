use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSTuple;

impl GTRender for TSTuple {
    fn render(&self, indent: &GTIndent) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent))
            .collect::<Vec<String>>()
            .join(", ");
        format!("{}{}{}", "[", descriptors, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor};

    #[test]
    fn test_render_tuple() {
        let indent = ts_indent();
        assert_eq!(
            TSTuple {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                    TSTypeDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(&indent),
            "[string, number]"
        );
    }
}
