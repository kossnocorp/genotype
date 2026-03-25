use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsTuple {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        Ok(format!("[{}]", descriptors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            TsTuple {
                descriptors: vec![
                    TsDescriptor::Primitive(TsPrimitive::String),
                    TsDescriptor::Primitive(TsPrimitive::Number),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"[string, number]"
        );
    }
}
