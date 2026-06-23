use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyModule {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
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
                .collect::<Result<Vec<_>, _>>()?,
        );

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let type_vars = self.render_type_vars(state, context)?;
        if !type_vars.is_empty() {
            blocks.push(type_vars);
        }

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(state, context))
                .collect::<Result<Vec<_>, _>>()?,
        );

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(Self::join_blocks(&blocks))
    }
}

impl PyModule {
    fn render_type_vars(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        if context.config.version != PyVersion::Legacy {
            return Ok("".into());
        }

        let mut generics = IndexSet::new();
        for definition in &self.definitions {
            generics.extend(definition.generics().iter().cloned());
        }

        generics
            .iter()
            .map(|generic| {
                let generic = generic.render(state, context)?;
                Ok(format!(r#"{generic} = TypeVar("{generic}")"#))
            })
            .collect::<PyRenderResult<Vec<_>>>()
            .map(|type_vars| type_vars.join("\n"))
    }
}

impl GtlRenderModule for PyModule {
    fn join_definitions(definitions: &[String]) -> String {
        definitions.join("\n\n\n")
    }

    fn join_blocks(blocks: &[String]) -> String {
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
                        dependency: PyDependencyIdent::Local(".path.to.module".into())
                    },
                    PyImport {
                        reference: PyImportReference::Named(vec![
                            PyImportName::Name("Name".into()),
                            PyImportName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: PyDependencyIdent::Local(".path.to.module".into())
                    }
                ],
                definitions: vec![
                    PyDefinition::Alias(PyAlias {
                        doc: None,
                        name: "Name".into(),
                        generics: vec![],
                        descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                        references: vec![],
                    }),
                    PyDefinition::Class(PyClass {
                        doc: None,
                        name: "Name".into(),
                        generics: vec![],
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
                    dependency: PyDependencyIdent::Local(".path.to.module".into())
                },],
                definitions: vec![PyDefinition::Alias(PyAlias {
                    doc: None,
                    name: "Name".into(),
                    generics: vec![],
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
