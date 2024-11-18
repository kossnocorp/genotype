use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSStruct;

impl RSRender for RSStruct {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent, config)?);
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(indent, config)?);
        }

        let name = self.name.render(indent, config)?;
        let fields = self.fields.render(indent, config)?;

        blocks.push(format!(
            "{indent}struct {name}{fields}",
            indent = indent.string
        ));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::GTAliasId;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            RSStruct {
                id: GTAliasId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "struct Name;"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            RSStruct {
                id: GTAliasId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                    }
                ]
                .into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"struct Name {
    name: String,
    age: isize,
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSStruct {
                id: GTAliasId("module".into(), "Person".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                    }
                ]
                .into(),
            }
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    struct Name {
        name: String,
        age: isize,
    }"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            RSStruct {
                id: GTAliasId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
struct Name;"#
        );
    }

    #[test]
    fn test_render_doc_fields() {
        assert_eq!(
            RSStruct {
                id: GTAliasId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                fields: vec![RSField {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }]
                .into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
struct Name {
    name: String,
}"#
        );
    }
}
