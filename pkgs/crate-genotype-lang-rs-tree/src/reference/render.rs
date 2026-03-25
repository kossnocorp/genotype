use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsReference {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
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
