use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSMap {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let key = self.key.render(context)?;
        let descriptor = self.descriptor.render(context)?;
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
            .render(&mut Default::default())
            .unwrap(),
            "BTreeMap<String, isize>"
        );
    }
}
