use super::RSAttribute;
use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSAttribute {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(context.indent_format(&format!("#[{content}]", content = self.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(&mut Default::default())
                .unwrap(),
            "#[derive]"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(&mut RSRenderContext::default().indent_inc())
                .unwrap(),
            "    #[derive]"
        );
    }
}
