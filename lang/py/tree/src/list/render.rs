use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::PYVersion;
use miette::Result;

impl<'a> GtlRender<'a> for PYList {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let list = if let PYVersion::Legacy = context.config.version {
            "List"
        } else {
            "list"
        };
        let descriptor = self.descriptor.render(state, context)?;

        Ok(format!("{list}[{descriptor}]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_lang_py_config::PYLangConfig;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "list[str]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            "List[str]"
        );
    }
}
