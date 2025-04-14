use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSRecord {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        Ok(format!("Record<{key}, {descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            TSRecord {
                key: TSRecordKey::Number,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Record<number, string>"
        );
    }
}
