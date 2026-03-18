use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSIntersection {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

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
            .join(" & "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            TSIntersection {
                descriptors: vec![
                    TSObject {
                        properties: vec![TSProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TSPrimitive::String.into(),
                            required: true,
                        }],
                    }
                    .into(),
                    "World".into(),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        {
          hello: string
        } & World
        "
        );
    }
}
