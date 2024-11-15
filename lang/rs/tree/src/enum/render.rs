use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSEnum;

impl RSRender for RSEnum {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent, config)?);
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(indent, config)?);
        }

        let name = self.name.render(indent, config)?;
        blocks.push(format!("{indent}enum {name} {{", indent = indent.string,));

        let variants_indent = indent.increment();
        for variant in &self.variants {
            blocks.push(variant.render(&variants_indent, config)?);
        }

        blocks.push(indent.format("}"));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSEnum {
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int).into(),
                    },
                ],
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSEnum {
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int).into(),
                    },
                ],
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    enum Union {
        String(String),
        Int(isize),
    }"#
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSEnum {
                doc: None,
                attributes: vec![RSAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int).into(),
                    },
                ],
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"#[derive(Deserialize, Serialize)]
enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSEnum {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int).into(),
                    },
                ],
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_eq!(
            RSEnum {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int).into(),
                    },
                ],
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    /// Hello, world!
    #[derive(Deserialize, Serialize)]
    enum Union {
        String(String),
        Int(isize),
    }"#
        );
    }
}
