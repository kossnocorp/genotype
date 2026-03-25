use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsExtension {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        self.reference.render(state, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsExtension {
                reference: "Foo".into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Foo"
        );
    }
}
