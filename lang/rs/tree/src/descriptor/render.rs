use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use super::{RSDescriptor, RSRender};

impl RSRender for RSDescriptor {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSDescriptor::Enum(r#enum) => r#enum.render(indent, config)?,
            RSDescriptor::Vec(array) => array.render(indent, config)?,
            RSDescriptor::Primitive(primitive) => primitive.render(indent, config)?,
            RSDescriptor::Reference(name) => name.render(indent, config)?,
            RSDescriptor::InlineUse(inline_use) => inline_use.render(indent, config)?,
            RSDescriptor::Tuple(tuple) => tuple.render(indent, config)?,
            RSDescriptor::HashMap(dict) => dict.render(indent, config)?,
            RSDescriptor::Option(option) => option.render(indent, config)?,
            RSDescriptor::Any(any) => any.render(indent, config)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::{GTDefinitionId, GTReferenceId};
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSDescriptor::Vec(Box::new(RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int)
            }))
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "Vec<isize>"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean)
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String)
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "String"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            RSDescriptor::Reference(RSReference {
                id: GTReferenceId("module".into(), (0, 0).into()),
                identifier: "Name".into(),
                definition_id: GTDefinitionId("module".into(), "Name".into())
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_inline_use() {
        assert_eq!(
            RSDescriptor::InlineUse(RSInlineUse {
                path: RSPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into()
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "HashMap<String, isize>"
        );
    }

    #[test]
    fn test_render_option() {
        assert_eq!(
            RSDescriptor::Option(Box::new(RSOption::new(RSDescriptor::Primitive(
                RSPrimitive::String
            ))))
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "Option<String>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            RSDescriptor::Any(RSAny)
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "Value"
        );
    }
}
