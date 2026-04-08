use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyTuple {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let tuple = if let PyVersion::Legacy = context.config.version {
            "Tuple"
        } else {
            "tuple"
        };

        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        let descriptors = if !descriptors.is_empty() {
            descriptors
        } else {
            "()".into()
        };

        Ok(format!("{tuple}[{descriptors}]",))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            PyTuple {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_snapshot!(
            PyTuple {
                descriptors: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_snapshot!(
            PyTuple {
                descriptors: vec![]
            }
            .render(
                Default::default(),
                &mut PyRenderContext {
                    config: &PyConfigLang::new(PyVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            @"Tuple[()]"
        );
    }
}
