use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSProperty;

impl RSRender for RSProperty {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent, config)?);
        }

        if self.attributes.len() > 0 {
            for attribute in &self.attributes {
                blocks.push(attribute.render(indent, config)?);
            }
        }

        let name = self.name.render(indent, config)?;
        let descriptor = self.descriptor.render(indent, config)?;
        blocks.push(format!(
            "{indent}{name}: {descriptor}",
            indent = indent.string
        ));

        Ok(blocks.join("\n"))
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "name: String"
        );
        assert_eq!(
            RSProperty {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSReference::new("Name".into()).into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    name: String"
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "name: String"
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
name: String"#
        );
        assert_eq!(
            RSProperty {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    /// Hello, world!
    name: String"#
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "#[derive(Clone)]
name: String"
        );
        assert_eq!(
            RSProperty {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    #[derive(Clone)]
    name: String"
        );
        assert_eq!(
            RSProperty {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    /// Hello, world!
    #[derive(Clone)]
    name: String"
        );
    }
}
