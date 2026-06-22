use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyDict {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let dict = if let PyVersion::Legacy = context.config.version {
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
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyDict {
                key: PyDictKey::String,
                descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"dict[str, int]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PyDict {
                key: PyDictKey::String,
                descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                }
            )
            .unwrap(),
            @"Dict[str, int]"
        );
    }
}
