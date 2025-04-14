use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSAny {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, _state: Self::RenderState, _context: &mut Self::RenderContext) -> Result<String> {
        Ok("any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(TSAny.render(Default::default(), &mut Default::default()).unwrap(), "any");
    }
}
