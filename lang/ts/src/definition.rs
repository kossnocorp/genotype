use genotype_lang_core::{indent::Indent, node::Node};

use crate::definition_descriptor::TSDefinitionDescriptor;

#[derive(Debug, PartialEq, Clone)]
pub struct TSDefinition {
    pub doc: Option<String>,
    pub descriptor: TSDefinitionDescriptor,
}

impl Node for TSDefinition {
    fn render(&self, indent: &Indent) -> String {
        format!("export {}", self.descriptor.render(&indent))
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
            TSDefinition {
                doc: None,
                descriptor: TSDefinitionDescriptor::Alias(TSAlias {
                    name: TSName("Name".to_string()),
                    descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                }),
            }
            .render(&indent),
            "export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        let indent = ts_indent();
        assert_eq!(
            TSDefinition {
                doc: None,
                descriptor: TSDefinitionDescriptor::Interface(TSInterface {
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
                }),
            }
            .render(&indent),
            "export interface Name {\n  name: string;\n  age?: number\n}"
        );
    }
}
