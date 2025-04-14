use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSTuple {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
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

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "[string, number]"
        );
    }
}
