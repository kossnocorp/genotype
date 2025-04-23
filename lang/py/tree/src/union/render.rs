use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYUnion {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let content = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(if let PYVersion::Legacy = context.config.version {
                ", "
            } else {
                " | "
            });

        let union = if let PYVersion::Legacy = context.config.version {
            format!("Union[{}]", content)
        } else {
            content
        };

        Ok(if let Some(discriminator) = &self.discriminator {
            format!(
                r#"Annotated[{}, Field(json_schema_extra={{'discriminator': '{}'}})]"#,
                union, discriminator
            )
        } else {
            union
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_union() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ],
                discriminator: None
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "str | int"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ],
                discriminator: None
            }
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            "Union[str, int]"
        );
    }

    #[test]
    fn test_render_discriminator() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ],
                discriminator: Some("type".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"Annotated[str | int, Field(json_schema_extra={'discriminator': 'type'})]"#
        );
    }

    #[test]
    fn test_render_discriminator_legacy() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ],
                discriminator: Some("type".into())
            }
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PYLangConfig::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            r#"Annotated[Union[str, int], Field(json_schema_extra={'discriminator': 'type'})]"#
        );
    }
}
