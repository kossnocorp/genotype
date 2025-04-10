use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSArray {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let descriptor = self.descriptor.render(context)?;
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
            .render(&mut Default::default())
            .unwrap(),
            "Array<string>"
        );
    }
}
