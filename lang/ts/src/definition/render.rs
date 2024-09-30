use genotype_lang_core::{indent::GTIndent, render::GTRender};

use super::TSDefinition;

impl GTRender for TSDefinition {
    fn render(&self, indent: &GTIndent) -> String {
        format!("export {}", self.descriptor.render(&indent))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        alias::TSAlias, definition_descriptor::TSDefinitionDescriptor, indent::ts_indent,
        interface::TSInterface, name::TSName, primitive::TSPrimitive, property::TSProperty,
        type_descriptor::TSTypeDescriptor,
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
