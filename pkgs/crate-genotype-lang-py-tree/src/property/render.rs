use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyProperty {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;

        let descriptor = self.descriptor.render(state, context)?;
        let descriptor = if self.required {
            descriptor
        } else {
            format!("Optional[{descriptor}] = None")
        };

        let doc = if let Some(doc) = &self.doc {
            format!("\n{}", doc.render(state, context)?)
        } else {
            "".into()
        };

        Ok(state.indent_format(&format!("{name}: {descriptor}{doc}",)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            PyProperty {
                doc: None,
                name: "name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: str"
        );
        assert_snapshot!(
            PyProperty {
                doc: None,
                name: "name".into(),
                descriptor: PyReference::new("Name".into(), false).into(),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            PyProperty {
                doc: None,
                name: "name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                required: true
            }
            .render(
                PyRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"    name: str"
        );
    }

    #[test]
    fn test_render_required() {
        assert_snapshot!(
            PyProperty {
                doc: None,
                name: "name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                required: false
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: Optional[str] = None"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            PyProperty {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                required: false
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        name: Optional[str] = None
        """Hello, world!"""
        "#
        );
    }
}
