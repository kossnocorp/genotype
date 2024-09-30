use genotype_lang_core::{indent::Indent, node::Node};

use crate::type_descriptor::TSTypeDescriptor;

#[derive(Debug, PartialEq, Clone)]
pub struct TSTuple {
    pub descriptors: Vec<TSTypeDescriptor>,
}

impl Node for TSTuple {
    fn render(&self, indent: &Indent) -> String {
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
    use crate::{indent::ts_indent, primitive::TSPrimitive};

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
