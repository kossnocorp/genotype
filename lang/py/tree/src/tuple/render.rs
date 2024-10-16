use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender, PYVersion};

use super::PYTuple;

impl PYRender for PYTuple {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, options))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "{}{}{}{}",
            if let PYVersion::Legacy = options.version {
                "Tuple"
            } else {
                "tuple"
            },
            "[",
            if descriptors.len() > 0 {
                descriptors
            } else {
                "()".into()
            },
            "]"
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive, PYVersion};

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
            "tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(&py_indent(), &PYOptions::default()),
            "tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(&py_indent(), &PYOptions::new(PYVersion::Legacy)),
            "Tuple[()]"
        );
    }
}
