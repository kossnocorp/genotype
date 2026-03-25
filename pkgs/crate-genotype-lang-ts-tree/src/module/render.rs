use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsModule {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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

        TsDoc::with_doc(&self.doc, state, context, str, true)
    }
}

impl GtlRenderModule for TsModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsModule {
                doc: None,
                imports: vec![
                    TsImport {
                        path: "../path/to/module".into(),
                        reference: TsImportReference::Default("Name".into()),
                    },
                    TsImport {
                        path: "../path/to/module".into(),
                        reference: TsImportReference::Named(vec![
                            TsImportName::Name("Name".into()),
                            TsImportName::Alias("Name".into(), "Alias".into()),
                        ])
                    }
                ],
                definitions: vec![
                    TsDefinition::Alias(TsAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                    }),
                    TsDefinition::Interface(TsInterface {
                        doc: None,
                        name: "Name".into(),
                        extensions: vec![],
                        properties: vec![
                            TsProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                                required: true
                            },
                            TsProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: TsDescriptor::Primitive(TsPrimitive::Number),
                                required: false
                            }
                        ]
                    }),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        import Name from "../path/to/module.js";
        import { Name, Name as Alias } from "../path/to/module.js";

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
        assert_snapshot!(
            TsModule {
                doc: Some(TsDoc("Hello, world!".into())),
                imports: vec![TsImport {
                    path: "../path/to/module".into(),
                    reference: TsImportReference::Default("Name".into()),
                },],
                definitions: vec![TsDefinition::Alias(TsAlias {
                    doc: None,
                    name: "Name".into(),
                    descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                }),]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        /** Hello, world! */

        import Name from "../path/to/module.js";

        export type Name = string;
        "#
        );
    }
}
