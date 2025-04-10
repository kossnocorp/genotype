use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSExtension {
    type RenderContext = TSRenderContext<'a>;

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
            TSExtension {
                reference: "Foo".into()
            }
            .render(&mut Default::default())
            .unwrap(),
            "Foo"
        );
    }
}
