use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender};

use super::PYDefinition;

impl PYRender for PYDefinition {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let definition = match self {
            PYDefinition::Alias(alias) => alias.render(indent, options),
            PYDefinition::Interface(interface) => interface.render(indent, options),
        };

        format!("export {}", definition)
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
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            })
            .render(&py_indent(), &PYOptions::default()),
            "export type Name = str;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_eq!(
            PYDefinition::Interface(PYInterface {
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ]
            })
            .render(&py_indent(), &PYOptions::default()),
            r#"export interface Name {
    name: str;
    age?: int;
}"#
        );
    }
}
