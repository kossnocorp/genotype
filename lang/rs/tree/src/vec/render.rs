use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSVec {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("Vec<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Vec<String>"
        );
    }
}
