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
    use crate::{descriptor::TSDescriptor, indent::ts_indent, primitive::TSPrimitive};

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(&ts_indent()),
            "[string, number]"
        );
    }
}
