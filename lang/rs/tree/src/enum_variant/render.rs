use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSEnumVariant {
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
        let descriptor = self.descriptor.render(state, context)?;
        blocks.push(state.indent_format(&format!("{name}({descriptor}),")));

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
            .render(Default::default(), &mut Default::default())
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
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
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
            .render(Default::default(), &mut Default::default())
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
            .render(Default::default(), &mut Default::default())
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
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    /// Hello, world!
    #[serde(rename = "variant")]
    Variant(bool),"#
        );
    }
}
