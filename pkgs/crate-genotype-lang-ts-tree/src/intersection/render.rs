use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsIntersection {
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
            TsIntersection {
                descriptors: vec![
                    TsObject {
                        properties: vec![TsProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TsPrimitive::String.into(),
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
