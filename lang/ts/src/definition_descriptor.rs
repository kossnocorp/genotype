use genotype_lang_core::{indent::Indent, node::Node};

use crate::{alias::TSAlias, interface::TSInterface};

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinitionDescriptor {
    Alias(TSAlias),
    Interface(TSInterface),
}

impl Node for TSDefinitionDescriptor {
    fn render(&self, indent: &Indent) -> String {
        match self {
            TSDefinitionDescriptor::Alias(alias) => alias.render(indent),
            TSDefinitionDescriptor::Interface(interface) => interface.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        alias::TSAlias, indent::ts_indent, interface::TSInterface, name::TSName,
        primitive::TSPrimitive, property::TSProperty, type_descriptor::TSTypeDescriptor,
    };

    #[test]
    fn test_render_alias() {
        let indent = ts_indent();
        assert_eq!(
            TSDefinitionDescriptor::Alias(TSAlias {
                name: TSName("Name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
            })
            .render(&indent),
            "type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        let indent = ts_indent();
        assert_eq!(
            TSDefinitionDescriptor::Interface(TSInterface {
                name: TSName("Name".to_string()),
                properties: vec![
                    TSProperty {
                        name: TSName("name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        optional: false
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        optional: true
                    }
                ]
            })
            .render(&indent),
            "interface Name {\n  name: string;\n  age?: number\n}"
        );
    }
}
