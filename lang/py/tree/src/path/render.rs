use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYPath {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _state: Self::RenderState, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYPath(".path.to.module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            ".path.to.module"
        );
    }
}
