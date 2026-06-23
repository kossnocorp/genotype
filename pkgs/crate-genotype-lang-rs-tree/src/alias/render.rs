use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsAlias {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generics = render_generics(&self.generics, state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        let r#type = format!("pub type {name}{generics} = {descriptor};");

        Ok(if let Some(doc) = &self.doc {
            format!("{}\n{}", doc.render(state, context)?, r#type)
        } else {
            r#type
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                generics: vec![],
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"pub type Name = String;"
        );
    }

    #[test]
    fn test_render_with_generics() {
        assert_snapshot!(
            RsAlias {
                id: GtDefinitionId("module".into(), "Response".into()),
                doc: None,
                name: "Response".into(),
                generics: vec!["Payload".into()],
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"pub type Response<Payload> = String;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                generics: vec![],
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /// Hello, world!
        pub type Name = String;
        "
        );
    }
}
