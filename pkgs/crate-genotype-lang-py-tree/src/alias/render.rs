use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyAlias {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        let alias = if let PyVersion::Legacy = context.config.version {
            format!("{name} = {descriptor}")
        } else {
            format!("type {name} = {descriptor}")
        };

        Ok(if let Some(doc) = &self.doc {
            let doc = doc.render(state, context)?;
            format!("{alias}\n{doc}")
        } else {
            alias
        })
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
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                references: vec![],
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                    ..Default::default()
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
}
