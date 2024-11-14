use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use super::{RSDescriptor, RSRender};

impl RSRender for RSDescriptor {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        match self {
            RSDescriptor::Enum(r#enum) => r#enum.render(indent, config),
            RSDescriptor::List(array) => array.render(indent, config),
            RSDescriptor::Primitive(primitive) => primitive.render(indent),
            RSDescriptor::Reference(name) => name.render(indent, config),
            RSDescriptor::InlineUse(inline_use) => inline_use.render(indent),
            RSDescriptor::Tuple(tuple) => tuple.render(indent, config),
            RSDescriptor::HashMap(dict) => dict.render(indent, config),
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
            RSDescriptor::List(Box::new(RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int)
            }))
            .render(&rs_indent(), &Default::default()),
            "Vec<isize>"
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
    fn test_render_inline_use() {
        assert_eq!(
            RSDescriptor::InlineUse(RSInlineUse {
                path: "self::path::to::module".into(),
                name: "Name".into()
            })
            .render(&rs_indent(), &Default::default()),
            "self::path::to::module::Name"
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
            "(isize, String)"
        );
    }

    #[test]
    fn test_render_hash_map() {
        assert_eq!(
            RSDescriptor::HashMap(Box::new(RSHashMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::Int.into(),
            }))
            .render(&rs_indent(), &Default::default()),
            "HashMap<String, isize>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            RSDescriptor::Any(RSAny).render(&rs_indent(), &Default::default()),
            "Value"
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
