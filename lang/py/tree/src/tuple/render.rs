use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYTuple {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let tuple = if let PYVersion::Legacy = context.config.version {
            "Tuple"
        } else {
            "tuple"
        };

        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        let descriptors = if descriptors.len() > 0 {
            descriptors
        } else {
            "()".into()
        };

        Ok(format!("{tuple}[{descriptors}]",))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_snapshot!(
            PYTuple {
                descriptors: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PYTuple {
                descriptors: vec![]
            }
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PyConfigLang::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            @"Tuple[()]"
        );
    }
}
