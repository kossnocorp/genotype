use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::PYVersion;
use miette::Result;

impl<'a> GtlRender<'a> for PYDict {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let dict = if let PYVersion::Legacy = context.config.version {
            "Dict"
        } else {
            "dict"
        };
        let key = self.key.render(context)?;
        let descriptor = self.descriptor.render(context)?;

        Ok(format!("{dict}[{key}, {descriptor}]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_lang_py_config::PYLangConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }
            .render(&mut Default::default())
            .unwrap(),
            "dict[str, int]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }
            .render(&mut PYRenderContext {
                config: &PYLangConfig::new(PYVersion::Legacy),
                ..Default::default()
            })
            .unwrap(),
            "Dict[str, int]"
        );
    }
}
