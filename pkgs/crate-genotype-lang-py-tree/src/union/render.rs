use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyUnion {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

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
            .join(if let PyVersion::Legacy = context.config.version {
                ", "
            } else {
                " | "
            });

        let union = if let PyVersion::Legacy = context.config.version {
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
    use insta::assert_snapshot;

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            PyUnion {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ],
                discriminator: None
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"str | int"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PyUnion {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ],
                discriminator: None
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            @"Union[str, int]"
        );
    }

    #[test]
    fn test_render_discriminator() {
        assert_snapshot!(
            PyUnion {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ],
                discriminator: Some("type".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Annotated[str | int, Field(json_schema_extra={'discriminator': 'type'})]"
        );
    }

    #[test]
    fn test_render_discriminator_legacy() {
        assert_snapshot!(
            PyUnion {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ],
                discriminator: Some("type".into())
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            @"Annotated[Union[str, int], Field(json_schema_extra={'discriminator': 'type'})]"
        );
    }
}
