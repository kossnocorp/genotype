use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYIdentifier {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYIdentifier("Foo".into())
                .render(&mut Default::default())
                .unwrap(),
            "Foo"
        );
    }
}
