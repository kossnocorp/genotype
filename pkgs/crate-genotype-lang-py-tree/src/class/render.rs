use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyClass {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generics = self.render_generics(state, context)?;
        let extensions = self.render_extensions(state, context)?;
        let body = self.render_body(state, context)?;

        Ok(state.indent_format(&format!("class {name}{generics}{extensions}:\n{body}")))
    }
}

impl<'a> PyClass {
    fn render_generics(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext<'a>,
    ) -> Result<String, PyRenderError> {
        if self.generics.is_empty() || context.config.version == PyVersion::Legacy {
            return Ok("".into());
        }

        Ok(format!(
            "[{}]",
            self.generics
                .iter()
                .map(|generic| generic.render(state, context))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ))
    }

    fn render_extensions(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext<'a>,
    ) -> Result<String, PyRenderError> {
        let mut extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(state, context))
            .collect::<Result<Vec<_>, _>>()?;
        // [TODO] Push model when converting instead
        extensions.push("Model".into());
        if context.config.version == PyVersion::Legacy && !self.generics.is_empty() {
            extensions.push(format!(
                "Generic[{}]",
                self.generics
                    .iter()
                    .map(|generic| generic.render(state, context))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ")
            ));
        }

        let extensions = extensions.join(", ");

        Ok(if !extensions.is_empty() {
            format!("({extensions})")
        } else {
            "".into()
        })
    }

    fn render_body(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext<'a>,
    ) -> Result<String, PyRenderError> {
        let mut body = vec![];

        if let Some(doc) = &self.doc {
            body.push(doc.render(state.indent_inc(), context)?);
        }

        if !self.properties.is_empty() {
            body.push(self.render_properties(state, context)?);
        } else {
            body.push(state.indent_inc().indent_format("pass"));
        }

        Ok(body.join("\n\n"))
    }

    fn render_properties(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext<'a>,
    ) -> Result<String, PyRenderError> {
        Ok(self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_empty() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        class Name(Model):
            pass
        "
        );
    }

    #[test]
    fn test_render_properties() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                extensions: vec![],
                properties: vec![
                    PyProperty {
                        doc: None,
                        name: "name".into(),
                        alias: None,
                        descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                        required: true
                    },
                    PyProperty {
                        doc: None,
                        name: "age".into(),
                        alias: None,
                        descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        class Name(Model):
            name: str
            age: Optional[int] = None
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                extensions: vec![],
                properties: vec![
                    PyProperty {
                        doc: None,
                        name: "name".into(),
                        alias: None,
                        descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                        required: true
                    },
                    PyProperty {
                        doc: None,
                        name: "age".into(),
                        alias: None,
                        descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            }
            .render(
                PyRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        class Name(Model):
            name: str
            age: Optional[int] = None
        "
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                extensions: vec![
                    PyReference::new("Hello".into(), false).into(),
                    PyReference::new("World".into(), false).into()
                ],
                properties: vec![PyProperty {
                    doc: None,
                    name: "name".into(),
                    alias: None,
                    descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                    required: true
                }],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        class Name(Hello, World, Model):
            name: str
        "
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_snapshot!(
            PyClass {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "Name".into(),
                generics: vec![],
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        class Name(Model):
            """Hello, world!"""

            pass
        "#
        );
    }

    #[test]
    fn test_render_doc_properties() {
        assert_snapshot!(
            PyClass {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "Name".into(),
                generics: vec![],
                extensions: vec![],
                properties: vec![PyProperty {
                    doc: None,
                    name: "name".into(),
                    alias: None,
                    descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                    required: true
                }],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        class Name(Model):
            """Hello, world!"""

            name: str
        "#
        );
    }

    #[test]
    fn test_render_with_generics() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Response".into(),
                generics: vec!["Payload".into()],
                extensions: vec![],
                properties: vec![PyProperty {
                    doc: None,
                    name: "value".into(),
                    alias: None,
                    descriptor: PyReference::new("Payload".into(), false).into(),
                    required: true,
                }],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        class Response[Payload](Model):
            value: Payload
        "
        );
    }

    #[test]
    fn test_render_legacy_with_generics() {
        assert_snapshot!(
            PyClass {
                doc: None,
                name: "Response".into(),
                generics: vec!["Payload".into()],
                extensions: vec![],
                properties: vec![PyProperty {
                    doc: None,
                    name: "value".into(),
                    alias: None,
                    descriptor: PyReference::new("Payload".into(), false).into(),
                    required: true,
                }],
                references: vec![],
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @"
        class Response(Model, Generic[Payload]):
            value: Payload
        "
        );
    }
}
