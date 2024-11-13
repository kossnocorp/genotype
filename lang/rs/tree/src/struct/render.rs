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
        let body = self.render_body(indent, config);
        // [TODO] Replace extensions with fields enum (resolved/unresolved)
        // let extensions = self.render_extensions(indent, config);

        blocks.push(format!("{}struct {name}{body}", indent.string));

        blocks.join("\n")
    }
}

impl RSStruct {
    fn render_extensions(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let mut extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(indent, config))
            .collect::<Vec<_>>();
        // [TODO] Push model when converting instead
        extensions.push("Model".into());

        let extensions = extensions.join(", ");

        if extensions.len() > 0 {
            format!("({extensions})")
        } else {
            "".into()
        }
    }

    fn render_body(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        if self.properties.len() == 0 {
            return ";".into();
        }

        let fields = self.render_fields(indent, config);
        format!(" {{\n{fields}\n{}}}", indent.string)
    }

    fn render_fields(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let indent = indent.increment();
        self.properties
            .iter()
            .map(|property| property.render(&indent, config) + ",")
            .collect::<Vec<String>>()
            .join("\n")
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
                extensions: vec![],
                properties: vec![],
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
                extensions: vec![],
                properties: vec![
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
                ],
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
                extensions: vec![],
                properties: vec![
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
                ],
            }
            .render(&rs_indent().increment(), &Default::default()),
            r#"    struct Name {
        name: String,
        age: isize,
    }"#
        );
    }

    #[test]
    #[ignore = "Extensions will be replaced by resolved/unresolved fields"]
    fn test_render_extensions() {
        assert_eq!(
            RSStruct {
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                extensions: vec![
                    RSReference::new("Hello".into()).into(),
                    RSReference::new("World".into()).into()
                ],
                properties: vec![RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }],
            }
            .render(&rs_indent(), &Default::default()),
            r#"class Name(Hello, World, Model):
    name: String"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            RSStruct {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
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
                extensions: vec![],
                properties: vec![RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }],
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
struct Name {
    name: String,
}"#
        );
    }
}
