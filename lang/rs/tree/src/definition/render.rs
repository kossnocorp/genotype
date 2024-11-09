use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSDefinition;

impl RSRender for RSDefinition {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        match self {
            RSDefinition::Alias(alias) => alias.render(indent, config),
            RSDefinition::Class(interface) => interface.render(indent, config),
        }
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
                references: vec![],
            })
            .render(&rs_indent(), &Default::default()),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_class() {
        assert_eq!(
            RSDefinition::Class(RSClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
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
                ],
                references: vec![],
            })
            .render(&rs_indent(), &Default::default()),
            r#"class Name(Model):
    name: str
    age: int"#
        );
    }
}
