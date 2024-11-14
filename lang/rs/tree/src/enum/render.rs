use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_core_tree::render::GTRender;
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSEnum;

impl RSRender for RSEnum {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent));
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(indent));
        }

        blocks.push(format!(
            "{}enum {} {{",
            indent.string,
            self.name.render(indent)
        ));

        let variants_indent = indent.increment();
        for variant in &self.variants {
            blocks.push(variant.render(&variants_indent, config));
        }

        blocks.push(indent.format("}"));

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
            .render(&rs_indent(), &Default::default()),
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
            .render(&rs_indent().increment(), &Default::default()),
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
            .render(&rs_indent(), &Default::default()),
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
            .render(&rs_indent(), &Default::default()),
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
            .render(&rs_indent().increment(), &Default::default()),
            r#"    /// Hello, world!
    #[derive(Deserialize, Serialize)]
    enum Union {
        String(String),
        Int(isize),
    }"#
        );
    }
}
