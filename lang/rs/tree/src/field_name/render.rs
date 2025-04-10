use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_rs_core::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSFieldName {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(RSNaming::render(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSFieldName("foo".into())
                .render(&mut Default::default())
                .unwrap(),
            "foo"
        );
    }

    #[test]
    fn test_render_keyword() {
        assert_eq!(
            RSFieldName("type".into())
                .render(&mut Default::default())
                .unwrap(),
            "r#type"
        );
    }
}
