use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsReference {

    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        self.identifier.render(state, context)
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
                definition_id: GtDefinitionId("module".into(), "Foo".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Foo"
        );
    }
}
