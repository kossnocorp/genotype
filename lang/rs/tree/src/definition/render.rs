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
    use genotype_parser::GTDefinitionId;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            })
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "pub type Name = String;"
        );
    }

    #[test]
    fn test_render_struct() {
        assert_eq!(
            RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSField {
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
            r#"pub struct Name {
    pub name: String,
    pub age: isize,
}"#
        );
    }

    #[test]
    fn test_render_enum() {
        assert_eq!(
            RSDefinition::Enum(RSEnum {
                id: GTDefinitionId("module".into(), "ValuesUnion".into()),
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
            r#"pub enum ValuesUnion {
    Boolean(bool),
    String(String),
}"#
        );
    }
}
