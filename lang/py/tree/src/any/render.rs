use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYAny {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok("Any".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(PYAny.render(&mut Default::default()).unwrap(), "Any");
    }
}
