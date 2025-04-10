use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSReference {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        self.0.render(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            TSReference("Foo".into())
                .render(&mut Default::default())
                .unwrap(),
            "Foo"
        );
    }
}
