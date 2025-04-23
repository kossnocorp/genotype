use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYDict {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let dict = if let PYVersion::Legacy = context.config.version {
            "Dict"
        } else {
            "dict"
        };
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        Ok(format!("{dict}[{key}, {descriptor}]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }
            .render(Default::default(), &mut Default::default())
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
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            "Dict[str, int]"
        );
    }
}
