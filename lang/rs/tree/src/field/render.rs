use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSField {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(context)?);
        }

        if self.attributes.len() > 0 {
            for attribute in &self.attributes {
                blocks.push(attribute.render(context)?);
            }
        }

        let name = self.name.render(context)?;
        let descriptor = self.descriptor.render(context)?;
        blocks.push(context.indent_format(&format!("pub {name}: {descriptor}")));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut Default::default())
            .unwrap(),
            "pub name: String"
        );
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSReference {
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    identifier: "Name".into(),
                    definition_id: GTDefinitionId("module".into(), "Name".into())
                }
                .into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            "pub name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            "    pub name: String"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub name: String"#
        );
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            r#"    /// Hello, world!
    pub name: String"#
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut Default::default())
            .unwrap(),
            "#[derive(Clone)]
pub name: String"
        );
        assert_eq!(
            RSField {
                doc: None,
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            "    #[derive(Clone)]
    pub name: String"
        );
        assert_eq!(
            RSField {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            "    /// Hello, world!
    #[derive(Clone)]
    pub name: String"
        );
    }
}
