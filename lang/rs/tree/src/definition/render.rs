use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSDefinition;

impl RSRender for RSDefinition {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSDefinition::Alias(alias) => alias.render(indent, config)?,
            RSDefinition::Struct(interface) => interface.render(indent, config)?,
            RSDefinition::Enum(r#enum) => r#enum.render(indent, config)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "type Name = String;"
        );
    }

    #[test]
    fn test_render_class() {
        assert_eq!(
            RSDefinition::Struct(RSStruct {
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                    }
                ]
                .into(),
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"struct Name {
    name: String,
    age: isize,
}"#
        );
    }

    #[test]
    fn test_render_enum() {
        assert_eq!(
            RSDefinition::Enum(RSEnum {
                doc: None,
                attributes: vec![],
                name: "ValuesUnion".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        name: "Boolean".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::Boolean).into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        name: "String".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::String).into()
                        ),
                    }
                ],
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"enum ValuesUnion {
    Boolean(bool),
    String(String),
}"#
        );
    }
}
