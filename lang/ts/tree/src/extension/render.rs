use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSExtension {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSExtension {
                reference: "Foo".into()
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Foo"
        );
    }
}
