use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSMap {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        Ok(format!("BTreeMap<{key}, {descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::IntSize.into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "BTreeMap<String, isize>"
        );
    }
}
