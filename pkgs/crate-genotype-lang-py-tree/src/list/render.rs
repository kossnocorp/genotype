use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyList {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let list = if let PyVersion::Legacy = context.config.version {
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
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            PyList {
                descriptor: PyDescriptor::Primitive(PyPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"list[str]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PyList {
                descriptor: PyDescriptor::Primitive(PyPrimitive::String)
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @"List[str]"
        );
    }
}
