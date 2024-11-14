use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_core_tree::render::GTRender;
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSEnumVariant;

impl RSRender for RSEnumVariant {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent));
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(indent));
        }

        let name = self.name.render(indent);
        let descriptor = self.descriptor.render(indent, config);
        blocks.push(format!(
            "{indent}{name}({descriptor}),",
            indent = indent.string
        ));

        blocks.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent(), &Default::default()),
            "Variant(bool),"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent().increment(), &Default::default()),
            "    Variant(bool),"
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![RSAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent(), &Default::default()),
            r#"#[serde(rename = "variant")]
Variant(bool),"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
Variant(bool),"#
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_eq!(
            RSEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent().increment(), &Default::default()),
            r#"    /// Hello, world!
    #[serde(rename = "variant")]
    Variant(bool),"#
        );
    }
}
