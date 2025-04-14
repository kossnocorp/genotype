use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSUnion {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
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

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "string | number"
        );
    }
}
