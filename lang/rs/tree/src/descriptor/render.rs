use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use super::{RSDescriptor, RSRender};

impl RSRender for RSDescriptor {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        match self {
            RSDescriptor::List(array) => array.render(indent, config),
            RSDescriptor::Literal(literal) => literal.render(indent),
            RSDescriptor::Primitive(primitive) => primitive.render(indent),
            RSDescriptor::Reference(name) => name.render(indent, config),
            RSDescriptor::Tuple(tuple) => tuple.render(indent, config),
            RSDescriptor::Union(union) => union.render(indent, config),
            RSDescriptor::Dict(dict) => dict.render(indent, config),
            RSDescriptor::Any(any) => any.render(indent),
            RSDescriptor::Option(option) => option.render(indent, config),
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
            RSDescriptor::List(Box::new(RSList {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int)
            }))
            .render(&rs_indent(), &Default::default()),
            "list[isize]"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean).render(&rs_indent(), &Default::default()),
            "bool"
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String).render(&rs_indent(), &Default::default()),
            "String"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            RSDescriptor::Reference(RSReference::new("Name".into()))
                .render(&rs_indent(), &Default::default()),
            "Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            RSDescriptor::Tuple(RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Int),
                    RSDescriptor::Primitive(RSPrimitive::String)
                ]
            })
            .render(&rs_indent(), &Default::default()),
            "tuple[isize, String]"
        );
    }

    #[test]
    fn test_render_union() {
        assert_eq!(
            RSDescriptor::Union(RSUnion {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::String),
                    RSDescriptor::Primitive(RSPrimitive::Int),
                ],
                discriminator: None
            })
            .render(&rs_indent(), &Default::default()),
            "String | isize"
        );
    }

    #[test]
    fn test_render_dict() {
        assert_eq!(
            RSDescriptor::Dict(Box::new(RSDict {
                key: RSDictKey::String,
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
            }))
            .render(&rs_indent(), &Default::default()),
            "dict[str, isize]"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            RSDescriptor::Any(RSAny).render(&rs_indent(), &Default::default()),
            "Any"
        );
    }

    #[test]
    fn test_render_option() {
        assert_eq!(
            RSDescriptor::Option(Box::new(RSOption::new(RSDescriptor::Primitive(
                RSPrimitive::String
            ))))
            .render(&rs_indent(), &Default::default()),
            "Option<String>"
        );
    }
}
