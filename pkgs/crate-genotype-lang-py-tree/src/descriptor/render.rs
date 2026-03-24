use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYDescriptor {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            PYDescriptor::List(array) => array.render(state, context),
            PYDescriptor::Literal(literal) => literal.render(state, context),
            PYDescriptor::Primitive(primitive) => primitive.render(state, context),
            PYDescriptor::Reference(name) => name.render(state, context),
            PYDescriptor::Tuple(tuple) => tuple.render(state, context),
            PYDescriptor::Union(union) => union.render(state, context),
            PYDescriptor::Dict(dict) => dict.render(state, context),
            PYDescriptor::Any(any) => any.render(state, context),
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
            PYDescriptor::List(Box::new(PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"list[int]"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            PYDescriptor::Primitive(PYPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PYDescriptor::Primitive(PYPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            PYDescriptor::Reference(PYReference::new("Name".into(), false))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            PYDescriptor::Tuple(PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Int),
                    PYDescriptor::Primitive(PYPrimitive::String)
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
            PYDescriptor::Union(PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
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
            PYDescriptor::Dict(Box::new(PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"dict[str, int]"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            PYDescriptor::Any(PYAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
