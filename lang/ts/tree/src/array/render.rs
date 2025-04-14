use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSArray {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("Array<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Array<string>"
        );
    }
}
