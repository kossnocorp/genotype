use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyNewtype {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(&self, state: PyRenderState, context: &mut PyRenderContext) -> Result<String> {
        let mut blocks = vec![];

        let name = self.name.render(state, context)?;
        let primitive = self.primitive.render(state, context)?;
        blocks.push(format!(r#"{name} = NewType("{name}", {primitive})"#));

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PyPrimitive::String,
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"UserId = NewType("UserId", str)"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            PyNewtype {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "UserId".into(),
                primitive: PyPrimitive::String,
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        UserId = NewType("UserId", str)
        """Hello, world!"""
        "#
        );
    }
}
