use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSProperty;

impl GTRender for TSProperty {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            "{}{}{}: {}",
            indent.string,
            self.name.render(indent),
            if self.required { "" } else { "?" },
            self.descriptor.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSProperty {
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(&ts_indent()),
            "name: string"
        );
        assert_eq!(
            TSProperty {
                name: "name".into(),
                descriptor: TSDescriptor::Reference("Name".into()),
                required: true
            }
            .render(&ts_indent()),
            "name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSProperty {
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(&ts_indent().increment()),
            "  name: string"
        );
    }

    #[test]
    fn test_render_required() {
        assert_eq!(
            TSProperty {
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: false
            }
            .render(&ts_indent()),
            "name?: string"
        );
    }
}
