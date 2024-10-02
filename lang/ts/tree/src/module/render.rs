use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSModule;

impl GTRender for TSModule {
    fn render(&self, indent: &GTIndent) -> String {
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
        alias::TSAlias, definition::TSDefinition, descriptor::TSDescriptor, import::TSImport,
        import_name::TSImportName, import_reference::TSImportReference, indent::ts_indent,
        interface::TSInterface, path::TSPath, primitive::TSPrimitive, property::TSProperty,
    };

    #[test]
    fn test_render() {
        assert_eq!(
            TSModule {
                doc: None,
                imports: vec![
                    TSImport {
                        path: TSPath::Resolved("../path/to/module.ts".into()),
                        reference: TSImportReference::Default("Name".into()),
                    },
                    TSImport {
                        path: TSPath::Resolved("../path/to/module.ts".into()),
                        reference: TSImportReference::Named(vec![
                            TSImportName::Name("Name".into()),
                            TSImportName::Alias("Name".into(), "Alias".into()),
                        ])
                    }
                ],
                definitions: vec![
                    TSDefinition::Alias(TSAlias {
                        name: "Name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    }),
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
                    }),
                ]
            }
            .render(&ts_indent()),
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
