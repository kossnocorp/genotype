use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsField {
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

        if !self.attributes.is_empty() {
            for attribute in &self.attributes {
                blocks.push(attribute.render(state, context)?);
            }
        }

        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        blocks.push(state.indent_format(&format!("pub {name}: {descriptor}")));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            RsField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"pub name: String"
        );
        assert_snapshot!(
            RsField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RsReference {
                    id: GtReferenceId("module".into(), (0, 0).into()),
                    identifier: "Name".into(),
                    definition_id: GtDefinitionId("module".into(), "Name".into())
                }
                .into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"pub name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RsField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"    pub name: String"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            RsField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /// Hello, world!
        pub name: String
        "
        );
        assert_snapshot!(
            RsField {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /// Hello, world!
        pub name: String
        "
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_snapshot!(
            RsField {
                doc: None,
                attributes: vec![RsAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        #[derive(Clone)]
        pub name: String
        "
        );
        assert_snapshot!(
            RsField {
                doc: None,
                attributes: vec![RsAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        #[derive(Clone)]
        pub name: String
        "
        );
        assert_snapshot!(
            RsField {
                doc: Some("Hello, world!".into()),
                attributes: vec![RsAttribute("derive(Clone)".into())],
                name: "name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /// Hello, world!
        #[derive(Clone)]
        pub name: String
        "
        );
    }
}
