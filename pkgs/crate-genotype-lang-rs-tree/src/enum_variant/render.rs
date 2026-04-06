use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsEnumVariant {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

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
        let descriptor = if let Some(descriptor) = &self.descriptor {
            format!("({})", descriptor.render(state, context)?)
        } else {
            "".into()
        };
        blocks.push(state.indent_format(&format!("{name}{descriptor},")));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: Some(RsDescriptor::Primitive(RsPrimitive::Boolean).into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Variant(bool),"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RsEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: Some(RsDescriptor::Primitive(RsPrimitive::Boolean).into()),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"    Variant(bool),"
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_snapshot!(
            RsEnumVariant {
                doc: None,
                attributes: vec![RsAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: Some(RsDescriptor::Primitive(RsPrimitive::Boolean).into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        #[serde(rename = "variant")]
        Variant(bool),
        "#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            RsEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Variant".into(),
                descriptor: Some(RsDescriptor::Primitive(RsPrimitive::Boolean).into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /// Hello, world!
        Variant(bool),
        "
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_snapshot!(
            RsEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![RsAttribute(r#"serde(rename = "variant")"#.into())],
                name: "Variant".into(),
                descriptor: Some(RsDescriptor::Primitive(RsPrimitive::Boolean).into()),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"
        /// Hello, world!
        #[serde(rename = "variant")]
        Variant(bool),
        "#
        );
    }

    #[test]
    fn test_render_no_descriptor() {
        assert_snapshot!(
            RsEnumVariant {
                doc: Some("Hello, world!".into()),
                attributes: vec![RsAttribute(r#"literal(3.14)"#.into())],
                name: "Variant".into(),
                descriptor: None,
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /// Hello, world!
        #[literal(3.14)]
        Variant,
        "
        );
    }
}
