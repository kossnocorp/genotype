use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyAlias {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generics = self.render_generics(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        let alias = if let PyVersion::Legacy = context.config.version {
            if self.generics.is_empty() {
                format!("{name} = {descriptor}")
            } else {
                format!("{name}: TypeAlias = {descriptor}")
            }
        } else {
            format!("type {name}{generics} = {descriptor}")
        };

        Ok(if let Some(doc) = &self.doc {
            let doc = doc.render(state, context)?;
            format!("{alias}\n{doc}")
        } else {
            alias
        })
    }
}

impl<'a> PyAlias {
    fn render_generics(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext<'a>,
    ) -> Result<String, PyRenderError> {
        if self.generics.is_empty() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyAlias {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"type Name = str"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PyAlias {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                references: vec![],
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @"Name = str"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            PyAlias {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "Name".into(),
                generics: vec![],
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        type Name = str
        """Hello, world!"""
        "#
        );
    }

    #[test]
    fn test_render_with_generics() {
        assert_snapshot!(
            PyAlias {
                doc: None,
                name: "Response".into(),
                generics: vec!["Payload".into()],
                descriptor: PyReference::new("Payload".into(), false).into(),
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"type Response[Payload] = Payload"
        );
    }

    #[test]
    fn test_render_legacy_with_generics() {
        assert_snapshot!(
            PyAlias {
                doc: None,
                name: "Response".into(),
                generics: vec!["Payload".into()],
                descriptor: PyReference::new("Payload".into(), false).into(),
                references: vec![],
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @"Response: TypeAlias = Payload"
        );
    }
}
