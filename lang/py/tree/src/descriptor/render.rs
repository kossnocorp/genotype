use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYDescriptor {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            PYDescriptor::List(array) => array.render(context),
            PYDescriptor::Literal(literal) => literal.render(context),
            PYDescriptor::Primitive(primitive) => primitive.render(context),
            PYDescriptor::Reference(name) => name.render(context),
            PYDescriptor::Tuple(tuple) => tuple.render(context),
            PYDescriptor::Union(union) => union.render(context),
            PYDescriptor::Dict(dict) => dict.render(context),
            PYDescriptor::Any(any) => any.render(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYDescriptor::List(Box::new(PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int)
            }))
            .render(&mut Default::default())
            .unwrap(),
            "list[int]"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            PYDescriptor::Primitive(PYPrimitive::Boolean)
                .render(&mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            PYDescriptor::Primitive(PYPrimitive::String)
                .render(&mut Default::default())
                .unwrap(),
            "str"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            PYDescriptor::Reference(PYReference::new("Name".into(), false))
                .render(&mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            PYDescriptor::Tuple(PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Int),
                    PYDescriptor::Primitive(PYPrimitive::String)
                ]
            })
            .render(&mut Default::default())
            .unwrap(),
            "tuple[int, str]"
        );
    }

    #[test]
    fn test_render_union() {
        assert_eq!(
            PYDescriptor::Union(PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ],
                discriminator: None
            })
            .render(&mut Default::default())
            .unwrap(),
            "str | int"
        );
    }

    #[test]
    fn test_render_dict() {
        assert_eq!(
            PYDescriptor::Dict(Box::new(PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }))
            .render(&mut Default::default())
            .unwrap(),
            "dict[str, int]"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            PYDescriptor::Any(PYAny)
                .render(&mut Default::default())
                .unwrap(),
            "Any"
        );
    }
}
