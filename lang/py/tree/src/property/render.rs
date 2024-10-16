use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender};

use super::PYProperty;

impl PYRender for PYProperty {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        format!(
            "{}{}{}: {}",
            indent.string,
            self.name.render(indent),
            if self.required { "" } else { "?" },
            self.descriptor.render(indent, options)
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: true
            }
            .render(&py_indent(), &PYOptions::default()),
            "name: str"
        );
        assert_eq!(
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Reference("Name".into()),
                required: true
            }
            .render(&py_indent(), &PYOptions::default()),
            "name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: true
            }
            .render(&py_indent().increment(), &PYOptions::default()),
            "    name: str"
        );
    }

    #[test]
    fn test_render_required() {
        assert_eq!(
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false
            }
            .render(&py_indent(), &PYOptions::default()),
            "name?: str"
        );
    }
}
