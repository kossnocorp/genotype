use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;

use super::{PYDescriptor, PYRender};

impl PYRender for PYDescriptor {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        match self {
            PYDescriptor::List(array) => array.render(indent, config),
            PYDescriptor::Literal(literal) => literal.render(indent),
            PYDescriptor::Primitive(primitive) => primitive.render(indent),
            PYDescriptor::Reference(name) => name.render(indent, config),
            PYDescriptor::Tuple(tuple) => tuple.render(indent, config),
            PYDescriptor::Union(union) => union.render(indent, config),
            PYDescriptor::Dict(dict) => dict.render(indent, config),
            PYDescriptor::Any(any) => any.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYDescriptor::List(Box::new(PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int)
            }))
            .render(&py_indent(), &Default::default()),
            "list[int]"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            PYDescriptor::Primitive(PYPrimitive::Boolean).render(&py_indent(), &Default::default()),
            "bool"
        );
        assert_eq!(
            PYDescriptor::Primitive(PYPrimitive::String).render(&py_indent(), &Default::default()),
            "str"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            PYDescriptor::Reference(PYReference::new("Name".into(), false))
                .render(&py_indent(), &Default::default()),
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
            .render(&py_indent(), &Default::default()),
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
            .render(&py_indent(), &Default::default()),
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
            .render(&py_indent(), &Default::default()),
            "dict[str, int]"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            PYDescriptor::Any(PYAny).render(&py_indent(), &Default::default()),
            "Any"
        );
    }
}
