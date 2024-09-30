use genotype_lang_core::{indent::Indent, node::Node};

use super::TSModule;

impl Node for TSModule {
    fn render(&self, indent: &Indent) -> String {
        let imports = self
            .imports
            .iter()
            .map(|import| import.render(indent))
            .collect::<Vec<String>>()
            .join("\n");

        let definitions = self
            .definitions
            .iter()
            .map(|definition| definition.render(indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        format!("{}\n\n{}\n", imports, definitions)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        alias::TSAlias, definition::TSDefinition, definition_descriptor::TSDefinitionDescriptor,
        import::TSImport, import_name::TSImportName, import_reference::TSImportReference,
        indent::ts_indent, interface::TSInterface, name::TSName, primitive::TSPrimitive,
        property::TSProperty, type_descriptor::TSTypeDescriptor,
    };

    #[test]
    fn test_render() {
        let indent = ts_indent();
        assert_eq!(
            TSModule {
                path: "./module.ts".to_string(),
                doc: None,
                imports: vec![
                    TSImport {
                        path: "../path/to/module.ts".to_string(),
                        reference: TSImportReference::Default(TSName("Name".to_string())),
                    },
                    TSImport {
                        path: "../path/to/module.ts".to_string(),
                        reference: TSImportReference::Named(vec![
                            TSImportName::Name(TSName("Name".to_string())),
                            TSImportName::Alias(
                                TSName("Name".to_string()),
                                TSName("Alias".to_string())
                            ),
                        ])
                    }
                ],
                definitions: vec![
                    TSDefinition {
                        doc: None,
                        descriptor: TSDefinitionDescriptor::Alias(TSAlias {
                            name: TSName("Name".to_string()),
                            descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        }),
                    },
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
                ]
            }
            .render(&indent),
            r#"import Name from "../path/to/module.ts";
import { Name, Name as Alias } from "../path/to/module.ts";

export type Name = string;

export interface Name {
  name: string;
  age?: number
}
"#
        );
    }
}
