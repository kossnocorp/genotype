use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender};

use super::PYModule;

impl PYRender for PYModule {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let imports = self
            .imports
            .iter()
            .map(|import| import.render(indent))
            .collect::<Vec<String>>()
            .join("\n");
        let has_imports = !imports.is_empty();

        let definitions = self
            .definitions
            .iter()
            .map(|definition| definition.render(indent, options))
            .collect::<Vec<String>>()
            .join("\n\n");
        let has_definitions = !definitions.is_empty();

        let mut str = imports;

        if has_imports && has_definitions {
            str.push_str("\n\n");
        }

        str.push_str(&definitions);

        if has_imports || has_definitions {
            str.push_str("\n");
        }

        str
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYModule {
                doc: None,
                imports: vec![
                    PYImport {
                        path: "../path/to/module.ts".into(),
                        reference: PYImportReference::Default("Name".into()),
                    },
                    PYImport {
                        path: "../path/to/module.ts".into(),
                        reference: PYImportReference::Named(vec![
                            PYImportName::Name("Name".into()),
                            PYImportName::Alias("Name".into(), "Alias".into()),
                        ])
                    }
                ],
                definitions: vec![
                    PYDefinition::Alias(PYAlias {
                        name: "Name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    }),
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
                    }),
                ]
            }
            .render(&py_indent(), &PYOptions::default()),
            r#"import Name from "../path/to/module.ts";
import { Name, Name as Alias } from "../path/to/module.ts";

export type Name = str;

export interface Name {
    name: str;
    age?: int;
}
"#
        );
    }
}
