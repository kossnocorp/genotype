use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSReference {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        self.0.render(state, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSReference("Foo".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Foo"
        );
    }
}
