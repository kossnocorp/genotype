use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSModule {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let imports = Self::join_imports(
            &self
                .imports
                .iter()
                .map(|import| import.render(state, context))
                .collect::<Result<Vec<_>>>()?,
        );
        let has_imports = !imports.is_empty();

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(state, context))
                .collect::<Result<Vec<_>>>()?,
        );
        let has_definitions = !definitions.is_empty();

        let mut str = imports;

        if has_imports && has_definitions {
            str.push_str("\n\n");
        }

        str.push_str(&definitions);

        if has_imports || has_definitions {
            str.push_str("\n");
        }

        TSDoc::with_doc(&self.doc, state, context, str, true)
    }
}

impl GtlRenderModule for TSModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSModule {
                doc: None,
                imports: vec![
                    TSImport {
                        path: "../path/to/module.ts".into(),
                        reference: TSImportReference::Default("Name".into()),
                    },
                    TSImport {
                        path: "../path/to/module.ts".into(),
                        reference: TSImportReference::Named(vec![
                            TSImportName::Name("Name".into()),
                            TSImportName::Alias("Name".into(), "Alias".into()),
                        ])
                    }
                ],
                definitions: vec![
                    TSDefinition::Alias(TSAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    }),
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
                    }),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"import Name from "../path/to/module.ts";
import { Name, Name as Alias } from "../path/to/module.ts";

export type Name = string;

export interface Name {
  name: string;
  age?: number;
}
"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSModule {
                doc: Some(TSDoc("Hello, world!".into())),
                imports: vec![TSImport {
                    path: "../path/to/module.ts".into(),
                    reference: TSImportReference::Default("Name".into()),
                },],
                definitions: vec![TSDefinition::Alias(TSAlias {
                    doc: None,
                    name: "Name".into(),
                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                }),]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/** Hello, world! */

import Name from "../path/to/module.ts";

export type Name = string;
"#
        );
    }
}
