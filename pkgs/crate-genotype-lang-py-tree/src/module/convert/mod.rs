use crate::prelude::internal::*;

mod ordering;
mod visitor;
pub(crate) use visitor::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PyConvertModule(pub PyModule);

impl PyConvertModule {
    pub fn convert(module: &GtModule, resolve: &PyConvertResolve, config: &PyConfig) -> Self {
        // [TODO] Get rid of unnecessary clone
        let mut context = PyConvertContext::new(resolve.clone(), config.clone());

        let doc = module.doc.as_ref().map(|doc| doc.convert(&mut context));

        for import in &module.imports {
            let import = import.convert(&mut context);
            context.push_import(import);
        }

        for alias in &module.aliases {
            let definition = alias.convert(&mut context);
            context.push_definition(definition);
        }

        let imports = context.drain_imports();

        let definitions = Self::sort_definitions(context.drain_definitions());

        let mut module = PyModule {
            doc,
            imports,
            definitions,
        };

        let mut visitor = PyModuleVisitor::new(&module);
        module.traverse(&mut visitor);

        PyConvertModule(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut resolve = PyConvertResolve::default();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_ron_snapshot!(
            PyConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GtImportReference::Glob((0, 0).into())
                        },
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GtImportReference::Names(
                                (0, 0).into(),
                                vec![
                                    GtImportName::Name(
                                        (0, 0).into(),
                                        GtIdentifier::new((0, 0).into(), "Name".into())
                                    ),
                                    GtImportName::Alias(
                                        (0, 0).into(),
                                        GtIdentifier::new((0, 0).into(), "Name".into()),
                                        GtIdentifier::new((0, 0).into(), "Alias".into())
                                    )
                                ]
                            )
                        }
                    ],
                    aliases: vec![
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Name".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: GtDescriptor::Object(GtObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GtIdentifier::new((0, 0).into(), "User".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GtProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GtKey::new((0, 0).into(), "name".into()),
                                        descriptor: Gt::primitive_string().into(),
                                        required: true,
                                    },
                                    GtProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GtKey::new((0, 0).into(), "age".into()),
                                        descriptor: Gt::primitive_i32().into(),
                                        required: false,
                                    }
                                ]
                            }),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Book".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: GtDescriptor::Object(GtObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GtIdentifier::new((0, 0).into(), "Order".into()).into(),
                                extensions: vec![],
                                properties: vec![GtProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GtKey::new((0, 0).into(), "book".into()),
                                    descriptor: GtDescriptor::Alias(Box::new(GtAlias {
                                        id: GtDefinitionId("module".into(), "Book".into()),
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GtIdentifier::new((0, 0).into(), "Book".into()),
                                        descriptor: GtDescriptor::Object(GtObject {
                                            span: (0, 0).into(),
                                            doc: None,
                                            attributes: vec![],
                                            name: GtIdentifier::new((0, 0).into(), "Book".into())
                                                .into(),
                                            extensions: vec![],
                                            properties: vec![
                                                GtProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GtKey::new((0, 0).into(), "title".into()),
                                                    descriptor: GtDescriptor::Primitive(
                                                        Gt::primitive_string()
                                                    ),
                                                    required: true,
                                                },
                                                GtProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GtKey::new(
                                                        (0, 0).into(),
                                                        "author".into()
                                                    ),
                                                    descriptor: Gt::reference("Author").into(),
                                                    required: true,
                                                }
                                            ]
                                        })
                                    })),
                                    required: true,
                                },]
                            }),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Order".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: Gt::primitive_string().into(),
                        },
                    ],
                },
                &resolve,
                &Default::default(),
            ),
            @r#"
        PyConvertModule(PyModule(
          doc: None,
          imports: [
            PyImport(
              dependency: Path(PyPath(".path.to.module")),
              reference: Default(Some(PyIdentifier("module"))),
            ),
            PyImport(
              dependency: Path(PyPath(".path.to.module")),
              reference: Named([
                Name(PyIdentifier("Name")),
                Alias(PyIdentifier("Name"), PyIdentifier("Alias")),
              ]),
            ),
            PyImport(
              dependency: Typing,
              reference: Named([
                Name(PyIdentifier("Optional")),
              ]),
            ),
            PyImport(
              dependency: Runtime,
              reference: Named([
                Name(PyIdentifier("Model")),
              ]),
            ),
          ],
          definitions: [
            Class(PyClass(
              doc: None,
              name: PyIdentifier("User"),
              extensions: [],
              properties: [
                PyProperty(
                  doc: None,
                  name: PyKey("name"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                PyProperty(
                  doc: None,
                  name: PyKey("age"),
                  descriptor: Primitive(Int),
                  required: false,
                ),
              ],
              references: [],
            )),
            Class(PyClass(
              doc: None,
              name: PyIdentifier("Book"),
              extensions: [],
              properties: [
                PyProperty(
                  doc: None,
                  name: PyKey("title"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                PyProperty(
                  doc: None,
                  name: PyKey("author"),
                  descriptor: Reference(PyReference(
                    identifier: PyIdentifier("Author"),
                    forward: true,
                  )),
                  required: true,
                ),
              ],
              references: [
                PyIdentifier("Author"),
              ],
            )),
            Class(PyClass(
              doc: None,
              name: PyIdentifier("Order"),
              extensions: [],
              properties: [
                PyProperty(
                  doc: None,
                  name: PyKey("book"),
                  descriptor: Reference(PyReference(
                    identifier: PyIdentifier("Book"),
                    forward: false,
                  )),
                  required: true,
                ),
              ],
              references: [
                PyIdentifier("Book"),
              ],
            )),
            Alias(PyAlias(
              doc: None,
              name: PyIdentifier("Name"),
              descriptor: Primitive(String),
              references: [],
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            PyConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default(),
            ),
            @r#"
        PyConvertModule(PyModule(
          doc: Some(PyDoc("Hello, world!")),
          imports: [],
          definitions: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_reorder() {
        assert_ron_snapshot!(
            PyConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Message".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Message".into()),
                            descriptor: Gt::descriptor(Gt::union(vec_into![
                                Gt::reference("DM"),
                                Gt::reference("Comment")
                            ]))
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "DM".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "DM".into()),
                            descriptor: GtObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GtIdentifier::new((0, 0).into(), "DM".into()).into(),
                                extensions: vec![],
                                properties: vec![GtProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GtKey::new((0, 0).into(), "message".into()),
                                    descriptor: Gt::primitive_string().into(),
                                    required: true,
                                }],
                            }
                            .into(),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Comment".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Comment".into()),
                            descriptor: GtObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GtIdentifier::new((0, 0).into(), "Comment".into()).into(),
                                extensions: vec![],
                                properties: vec![GtProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GtKey::new((0, 0).into(), "message".into()),
                                    descriptor: Gt::primitive_string().into(),
                                    required: true,
                                }],
                            }
                            .into(),
                        }
                    ],
                },
                &Default::default(),
                &Default::default(),
            ),
            @r#"
        PyConvertModule(PyModule(
          doc: None,
          imports: [
            PyImport(
              dependency: Runtime,
              reference: Named([
                Name(PyIdentifier("Model")),
              ]),
            ),
          ],
          definitions: [
            Class(PyClass(
              doc: None,
              name: PyIdentifier("DM"),
              extensions: [],
              properties: [
                PyProperty(
                  doc: None,
                  name: PyKey("message"),
                  descriptor: Primitive(String),
                  required: true,
                ),
              ],
              references: [],
            )),
            Class(PyClass(
              doc: None,
              name: PyIdentifier("Comment"),
              extensions: [],
              properties: [
                PyProperty(
                  doc: None,
                  name: PyKey("message"),
                  descriptor: Primitive(String),
                  required: true,
                ),
              ],
              references: [],
            )),
            Alias(PyAlias(
              doc: None,
              name: PyIdentifier("Message"),
              descriptor: Union(PyUnion(
                descriptors: [
                  Reference(PyReference(
                    identifier: PyIdentifier("DM"),
                    forward: false,
                  )),
                  Reference(PyReference(
                    identifier: PyIdentifier("Comment"),
                    forward: false,
                  )),
                ],
                discriminator: None,
              )),
              references: [
                PyIdentifier("DM"),
                PyIdentifier("Comment"),
              ],
            )),
          ],
        ))
        "#
        );
    }
}
