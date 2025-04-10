use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::PYVersion;
use miette::Result;

impl<'a> GtlRender<'a> for PYReference {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let str = self.identifier.render(context)?;
        if let PYVersion::Legacy = context.config.version {
            if self.forward {
                return Ok(format!("\"{str}\""));
            }
        }
        Ok(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_lang_py_config::PYLangConfig;

    #[test]
    fn test_render() {
        assert_eq!(
            PYReference::new("Foo".into(), false)
                .render(&mut Default::default())
                .unwrap(),
            "Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_eq!(
            PYReference::new("Foo".into(), true)
                .render(&mut Default::default())
                .unwrap(),
            "Foo"
        );
        assert_eq!(
            PYReference::new("Foo".into(), true)
                .render(&mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                })
                .unwrap(),
            "\"Foo\""
        );
    }
}
