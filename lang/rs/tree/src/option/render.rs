use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSOption {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let descriptor = self.descriptor.render(context)?;
        Ok(format!("Option<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSOption {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(&mut RSRenderContext::default())
            .unwrap(),
            "Option<String>"
        );
    }
}
