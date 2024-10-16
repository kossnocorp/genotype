use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender, PYVersion};

use super::PYList;

impl PYRender for PYList {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        format!(
            "{}[{}]",
            if let PYVersion::Legacy = options.version {
                "List"
            } else {
                "list"
            },
            self.descriptor.render(indent, options)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive, PYVersion};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYOptions::default()),
            "list[str]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYOptions::new(PYVersion::Legacy)),
            "List[str]"
        );
    }
}
