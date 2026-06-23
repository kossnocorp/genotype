use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsReference {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let reference = self.identifier.render(state, context)?;
        let arguments = render_generic_arguments(&self.arguments, state, context)?;
        Ok(format!("{reference}{arguments}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsReference {
                id: GtReferenceId("module".into(), (0, 0).into()),
                identifier: "Foo".into(),
                arguments: vec![],
                definition_id: GtDefinitionId("module".into(), "Foo".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Foo"
        );
    }

    #[test]
    fn test_render_with_arguments() {
        assert_snapshot!(
            RsReference {
                id: GtReferenceId("module".into(), (0, 0).into()),
                identifier: "Response".into(),
                arguments: vec![RsDescriptor::Primitive(RsPrimitive::String)],
                definition_id: GtDefinitionId("module".into(), "Response".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Response<String>"
        );
    }
}
