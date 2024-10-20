use genotype_config::GTConfig;
use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::PYRender;

use super::PYProperty;

impl PYRender for PYProperty {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
        let descriptor = self.descriptor.render(indent, config);

        let descriptor = if self.required {
            descriptor
        } else {
            format!("Optional[{descriptor}] = None")
        };

        format!(
            "{}{}: {}",
            indent.string,
            self.name.render(indent),
            descriptor
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
            .render(&py_indent(), &Default::default()),
            "name: str"
        );
        assert_eq!(
            PYProperty {
                name: "name".into(),
                descriptor: PYReference::new("Name".into(), false).into(),
                required: true
            }
            .render(&py_indent(), &Default::default()),
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
            .render(&py_indent().increment(), &Default::default()),
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
            .render(&py_indent(), &Default::default()),
            "name: Optional[str] = None"
        );
    }
}
