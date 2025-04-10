use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSInlineUse {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let module = self.path.render(context)?;
        let name = self.name.render(context)?;
        Ok(format!("{module}::{name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSInlineUse {
                path: RSPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into(),
            }
            .render(&mut Default::default())
            .unwrap(),
            "self::path::to::module::Name"
        );
    }
}
