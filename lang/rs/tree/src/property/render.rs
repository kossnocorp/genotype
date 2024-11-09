use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSProperty;

impl RSRender for RSProperty {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let descriptor = self.descriptor.render(indent, config);

        format!(
            "{}{}{}{}: {}",
            if let Some(doc) = &self.doc {
                format!("{}\n", doc.render(indent))
            } else {
                "".into()
            },
            if self.attributes.len() > 0 {
                let attributes = self
                    .attributes
                    .iter()
                    .map(|attr| attr.render(indent))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{attributes}\n")
            } else {
                "".into()
            },
            indent.string,
            self.name.render(indent),
            descriptor,
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
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            "name: str"
        );
        assert_eq!(
            RSProperty {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSReference::new("Name".into()).into(),
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
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
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
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            "name: str"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSProperty {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
name: str"#
        );
        assert_eq!(
            RSProperty {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default()),
            r#"    /// Hello, world!
    name: str"#
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSProperty {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            "#[derive(Clone)]
name: str"
        );
        assert_eq!(
            RSProperty {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default()),
            "    #[derive(Clone)]
    name: str"
        );
        assert_eq!(
            RSProperty {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default()),
            "    /// Hello, world!
    #[derive(Clone)]
    name: str"
        );
    }
}
