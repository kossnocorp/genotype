use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYDefinition;

impl PYRender for PYDefinition {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        match self {
            PYDefinition::Alias(alias) => alias.render(indent, config),
            PYDefinition::Class(interface) => interface.render(indent, config),
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
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                references: vec![],
            })
            .render(&py_indent(), &Default::default()),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_class() {
        assert_eq!(
            PYDefinition::Class(PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            })
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }
}
