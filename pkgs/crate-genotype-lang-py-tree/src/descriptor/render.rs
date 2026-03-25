use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyDescriptor {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            PyDescriptor::List(array) => array.render(state, context),
            PyDescriptor::Literal(literal) => literal.render(state, context),
            PyDescriptor::Primitive(primitive) => primitive.render(state, context),
            PyDescriptor::Reference(name) => name.render(state, context),
            PyDescriptor::Tuple(tuple) => tuple.render(state, context),
            PyDescriptor::Union(union) => union.render(state, context),
            PyDescriptor::Dict(dict) => dict.render(state, context),
            PyDescriptor::Any(any) => any.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            PyDescriptor::List(Box::new(PyList {
                descriptor: PyDescriptor::Primitive(PyPrimitive::Int)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"list[int]"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            PyDescriptor::Primitive(PyPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PyDescriptor::Primitive(PyPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            PyDescriptor::Reference(PyReference::new("Name".into(), false))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            PyDescriptor::Tuple(PyTuple {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::Int),
                    PyDescriptor::Primitive(PyPrimitive::String)
                ]
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"tuple[int, str]"
        );
    }

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            PyDescriptor::Union(PyUnion {
                descriptors: vec![
                    PyDescriptor::Primitive(PyPrimitive::String),
                    PyDescriptor::Primitive(PyPrimitive::Int),
                ],
                discriminator: None
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"str | int"
        );
    }

    #[test]
    fn test_render_dict() {
        assert_snapshot!(
            PyDescriptor::Dict(Box::new(PyDict {
                key: PyDictKey::String,
                descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"dict[str, int]"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            PyDescriptor::Any(PyAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
