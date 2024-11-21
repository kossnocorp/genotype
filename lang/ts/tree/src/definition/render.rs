use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDefinition;

impl GTRender for TSDefinition {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSDefinition::Alias(alias) => alias.render(indent),
            TSDefinition::Interface(interface) => interface.render(indent),
            TSDefinition::Branded(branded) => branded.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
            })
            .render(&ts_indent()),
            "export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_eq!(
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            })
            .render(&ts_indent()),
            r#"export interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_branded() {
        assert_eq!(
            TSDefinition::Branded(TSBranded {
                doc: None,
                name: "Version".into(),
                primitive: TSPrimitive::Number
            })
            .render(&ts_indent()),
            r#"export type Version = number & { [versionBrand]: true };
declare const versionBrand: unique symbol;"#
        );
    }
}
