use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSEnumVariant {
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
        let descriptor = self.descriptor.render(context)?;
        blocks.push(context.indent_format(&format!("{name}({descriptor}),")));

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
            RSEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            "Variant(bool),"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            "    Variant(bool),"
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![RSAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"#[serde(rename = "variant")]
Variant(bool),"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
Variant(bool),"#
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_eq!(
            RSEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&mut RSRenderContext::default().indent_inc())
            .unwrap(),
            r#"    /// Hello, world!
    #[serde(rename = "variant")]
    Variant(bool),"#
        );
    }
}
