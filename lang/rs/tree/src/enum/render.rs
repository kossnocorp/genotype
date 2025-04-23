use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSEnum {
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
        blocks.push(state.indent_format(&format!("pub enum {name} {{")));

        for variant in &self.variants {
            blocks.push(variant.render(state.indent_inc(), context)?);
        }

        blocks.push(state.indent_format("}"));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize).into(),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"pub enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize).into(),
                    },
                ],
            }
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    pub enum Union {
        String(String),
        Int(isize),
    }"#
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize).into(),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"#[derive(Deserialize, Serialize)]
pub enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize).into(),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub enum Union {
    String(String),
    Int(isize),
}"#
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_eq!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
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
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize).into(),
                    },
                ],
            }
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    /// Hello, world!
    #[derive(Deserialize, Serialize)]
    pub enum Union {
        String(String),
        Int(isize),
    }"#
        );
    }
}
