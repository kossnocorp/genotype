use genotype_lang_core::{indent::Indent, node::Node};

use crate::type_descriptor::TSTypeDescriptor;

#[derive(Debug, PartialEq, Clone)]
pub struct TSUnion {
    pub descriptors: Vec<TSTypeDescriptor>,
}

impl Node for TSUnion {
    fn render(&self, indent: &Indent) -> String {
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
    use crate::{indent::ts_indent, primitive::TSPrimitive};

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
