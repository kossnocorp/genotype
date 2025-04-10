use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSEnum {
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
        blocks.push(context.indent_format(&format!("pub enum {name} {{")));

        let mut variants_context = context.indent_inc();
        for variant in &self.variants {
            blocks.push(variant.render(&mut variants_context)?);
        }

        blocks.push(context.indent_format("}"));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GTDefinitionId;
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
            .render(&mut Default::default())
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
            .render(&mut RSRenderContext::default().indent_inc())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut RSRenderContext::default().indent_inc())
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
