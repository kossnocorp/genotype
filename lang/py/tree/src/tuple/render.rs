use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::*;
use miette::Result;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            "Tuple[()]"
        );
    }
}
