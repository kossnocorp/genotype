use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYExtension {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        self.reference.render(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYExtension {
                reference: PYReference::new("Foo".into(), false)
            }
            .render(&mut Default::default())
            .unwrap(),
            "Foo"
        );
    }
}
