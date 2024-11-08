use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSProperty;

impl RSRender for RSProperty {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let descriptor = self.descriptor.render(indent, config);

        let descriptor = if self.required {
            descriptor
        } else {
            format!("Optional[{descriptor}] = None")
        };

        format!(
            "{}{}: {}{}",
            indent.string,
            self.name.render(indent),
            descriptor,
            if let Some(doc) = &self.doc {
                format!("\n{}", doc.render(indent))
            } else {
                "".into()
            }
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
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                required: true
            }
            .render(&rs_indent(), &Default::default()),
            "name: str"
        );
        assert_eq!(
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSReference::new("Name".into()).into(),
                required: true
            }
            .render(&rs_indent(), &Default::default()),
            "name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                required: true
            }
            .render(&rs_indent().increment(), &Default::default()),
            "    name: str"
        );
    }

    #[test]
    fn test_render_required() {
        assert_eq!(
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                required: false
            }
            .render(&rs_indent(), &Default::default()),
            "name: Optional[str] = None"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSProperty {
                doc: Some(RSDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                required: false
            }
            .render(&rs_indent(), &Default::default()),
            r#"name: Optional[str] = None
"""Hello, world!""""#
        );
    }
}
