use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSStruct {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(context)?);
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(context)?);
        }

        let name = self.name.render(context)?;
        let fields = self.fields.render(context)?;

        blocks.push(context.indent_format(&format!("pub struct {name}{fields}")));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GTDefinitionId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            "pub struct Name;"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                    }
                ]
                .into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"pub struct Name {
    pub name: String,
    pub age: isize,
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                    }
                ]
                .into(),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            r#"    pub struct Name {
        pub name: String,
        pub age: isize,
    }"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub struct Name;"#
        );
    }

    #[test]
    fn test_render_doc_fields() {
        assert_eq!(
            RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
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
            .render(&mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub struct Name {
    pub name: String,
}"#
        );
    }
}
