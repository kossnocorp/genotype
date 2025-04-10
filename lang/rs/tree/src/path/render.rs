use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSPath {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GTModuleId;

    #[test]
    fn test_render() {
        assert_eq!(
            RSPath(
                GTModuleId("path/to/module".into()),
                "self::path::to::module".into()
            )
            .render(&mut Default::default())
            .unwrap(),
            "self::path::to::module"
        );
    }
}
