use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSStruct {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(state, context)?);
        }

        let name = self.name.render(state, context)?;
        let fields = self.fields.render(state, context)?;

        blocks.push(state.indent_format(&format!("pub struct {name}{fields}")));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "pub struct Name {}"
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
            .render(Default::default(), &mut Default::default())
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
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub struct Name {}"#
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub struct Name {
    pub name: String,
}"#
        );
    }
}
