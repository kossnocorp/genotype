use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYExtension {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

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

    #[test]
    fn test_render() {
        assert_eq!(
            PYExtension {
                reference: PYReference::new("Foo".into(), false)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Foo"
        );
    }
}
