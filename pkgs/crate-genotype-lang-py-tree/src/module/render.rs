use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyModule {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        blocks.push("from __future__ import annotations".into());

        let imports = Self::join_imports(
            &self
                .imports
                .iter()
                .map(|import| import.render(state, context))
                .collect::<Result<Vec<_>>>()?,
        );

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(state, context))
                .collect::<Result<Vec<_>>>()?,
        );

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(Self::join_blocks(&blocks))
    }
}

impl GtlRenderModule for PyModule {
    fn join_definitions(definitions: &Vec<String>) -> String {
        definitions.join("\n\n\n")
    }

    fn join_blocks(blocks: &Vec<String>) -> String {
        blocks.join("\n\n\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PyModule {
                doc: None,
                imports: vec![
                    PyImport {
                        reference: PyImportReference::Default(Some("name".into())),
                        dependency: PyDependencyIdent::Path(".path.to.module".into())
                    },
                    PyImport {
                        reference: PyImportReference::Named(vec![
                            PyImportName::Name("Name".into()),
                            PyImportName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: PyDependencyIdent::Path(".path.to.module".into())
                    }
                ],
                definitions: vec![
                    PyDefinition::Alias(PyAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                        references: vec![],
                    }),
                    PyDefinition::Class(PyClass {
                        doc: None,
                        name: "Name".into(),
                        extensions: vec![],
                        properties: vec![
                            PyProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                                required: true
                            },
                            PyProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
                                required: false
                            }
                        ],
                        references: vec![],
                    }),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        from __future__ import annotations


        import .path.to.module as name
        from .path.to.module import Name, Name as Alias


        type Name = str


        class Name(Model):
            name: str
            age: Optional[int] = None
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            PyModule {
                doc: Some(PyDoc("Hello, world!".into())),
                imports: vec![PyImport {
                    reference: PyImportReference::Default(Some("name".into())),
                    dependency: PyDependencyIdent::Path(".path.to.module".into())
                },],
                definitions: vec![PyDefinition::Alias(PyAlias {
                    doc: None,
                    name: "Name".into(),
                    descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                    references: vec![],
                })]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        """Hello, world!"""


        from __future__ import annotations


        import .path.to.module as name


        type Name = str
        "#
        );
    }
}
