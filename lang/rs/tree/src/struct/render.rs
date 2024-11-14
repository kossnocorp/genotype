use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSStruct;

impl RSRender for RSStruct {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(&indent));
        }

        let name = self.name.render(indent);
        let fields = self.fields.render(indent, config);

        blocks.push(format!("{}struct {name}{fields}", indent.string));

        blocks.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            RSStruct {
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&rs_indent(), &Default::default()),
            "struct Name;"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            RSStruct {
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                    }
                ]
                .into(),
            }
            .render(&rs_indent(), &Default::default()),
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
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                    }
                ]
                .into(),
            }
            .render(&rs_indent().increment(), &Default::default()),
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
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
struct Name;"#
        );
    }

    #[test]
    fn test_render_doc_fields() {
        assert_eq!(
            RSStruct {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                fields: vec![RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }]
                .into(),
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
struct Name {
    name: String,
}"#
        );
    }
}
