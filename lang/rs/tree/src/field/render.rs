use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSField;

impl RSRender for RSField {
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
            "{indent}pub {name}: {descriptor}",
            indent = indent.string
        ));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::{GTDefinitionId, GTReferenceId};
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "pub name: String"
        );
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSReference {
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    identifier: "Name".into(),
                    definition_id: GTDefinitionId("module".into(), "Name".into())
                }
                .into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "pub name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    pub name: String"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub name: String"#
        );
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    /// Hello, world!
    pub name: String"#
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "#[derive(Clone)]
pub name: String"
        );
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    #[derive(Clone)]
    pub name: String"
        );
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            "    /// Hello, world!
    #[derive(Clone)]
    pub name: String"
        );
    }
}
