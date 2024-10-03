use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDefinition;

impl GTRender for TSDefinition {
    fn render(&self, indent: &GTIndent) -> String {
        let definition = match self {
            TSDefinition::Alias(alias) => alias.render(indent),
            TSDefinition::Interface(interface) => interface.render(indent),
        };

        format!("export {}", definition)
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
                name: "Name".into(),
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            })
            .render(&ts_indent()),
            "export interface Name {\n  name: string;\n  age?: number\n}"
        );
    }
}
