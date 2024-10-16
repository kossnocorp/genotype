use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender};

use super::PYTuple;

impl PYRender for PYTuple {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, options))
            .collect::<Vec<String>>()
            .join(", ");
        format!("{}{}{}", "[", descriptors, "]")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive};

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(&py_indent(), &PYOptions::default()),
            "[str, int]"
        );
    }
}
