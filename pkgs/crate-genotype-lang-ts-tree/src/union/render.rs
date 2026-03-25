use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsUnion {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(" | "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            TsUnion {
                descriptors: vec![
                    TsDescriptor::Primitive(TsPrimitive::String),
                    TsDescriptor::Primitive(TsPrimitive::Number),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"string | number"
        );
    }
}
