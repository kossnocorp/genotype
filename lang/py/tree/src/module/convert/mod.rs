use crate::prelude::internal::*;

mod ordering;
mod visitor;
pub(crate) use visitor::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYConvertModule(pub PYModule);

impl PYConvertModule {
    pub fn convert(module: &GTModule, resolve: &PYConvertResolve, config: &PyConfig) -> Self {
        // [TODO] Get rid of unnecessary clone
        let mut context = PYConvertContext::new(resolve.clone(), config.clone());

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

        let mut module = PYModule {
            doc,
            imports,
            definitions,
        };

        let mut visitor = PYModuleVisitor::new(&module);
        module.traverse(&mut visitor);

        PYConvertModule(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut resolve = PYConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_ron_snapshot!(
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GTImportReference::Glob((0, 0).into())
                        },
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GTImportReference::Names(
                                (0, 0).into(),
                                vec![
                                    GTImportName::Name(
                                        (0, 0).into(),
                                        GTIdentifier::new((0, 0).into(), "Name".into())
                                    ),
                                    GTImportName::Alias(
                                        (0, 0).into(),
                                        GTIdentifier::new((0, 0).into(), "Name".into()),
                                        GTIdentifier::new((0, 0).into(), "Alias".into())
                                    )
                                ]
                            )
                        }
                    ],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Name".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((0, 0).into(), "User".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((0, 0).into(), "name".into()),
                                        descriptor: GtFactory::primitive_string().into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((0, 0).into(), "age".into()),
                                        descriptor: GtFactory::primitive_i32().into(),
                                        required: false,
                                    }
                                ]
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Book".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((0, 0).into(), "Order".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "book".into()),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        id: GTDefinitionId("module".into(), "Book".into()),
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTIdentifier::new((0, 0).into(), "Book".into()),
                                        descriptor: GTDescriptor::Object(GTObject {
                                            span: (0, 0).into(),
                                            doc: None,
                                            attributes: vec![],
                                            name: GTIdentifier::new((0, 0).into(), "Book".into())
                                                .into(),
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new((0, 0).into(), "title".into()),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GtFactory::primitive_string()
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (0, 0).into(),
                                                        "author".into()
                                                    ),
                                                    descriptor: GtFactory::reference("Author").into(),
                                                    required: true,
                                                }
                                            ]
                                        })
                                    })),
                                    required: true,
                                },]
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Order".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: GtFactory::primitive_string().into(),
                        },
                    ],
                },
                &resolve,
                &Default::default(),
            ),
            @r#"
        PYConvertModule(PYModule(
          doc: None,
          imports: [
            PYImport(
              dependency: Path(PYPath(".path.to.module")),
              reference: Default(Some(PYIdentifier("module"))),
            ),
            PYImport(
              dependency: Path(PYPath(".path.to.module")),
              reference: Named([
                Name(PYIdentifier("Name")),
                Alias(PYIdentifier("Name"), PYIdentifier("Alias")),
              ]),
            ),
            PYImport(
              dependency: Typing,
              reference: Named([
                Name(PYIdentifier("Optional")),
              ]),
            ),
            PYImport(
              dependency: Runtime,
              reference: Named([
                Name(PYIdentifier("Model")),
              ]),
            ),
          ],
          definitions: [
            Class(PYClass(
              doc: None,
              name: PYIdentifier("User"),
              extensions: [],
              properties: [
                PYProperty(
                  doc: None,
                  name: PYKey("name"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                PYProperty(
                  doc: None,
                  name: PYKey("age"),
                  descriptor: Primitive(Int),
                  required: false,
                ),
              ],
              references: [],
            )),
            Class(PYClass(
              doc: None,
              name: PYIdentifier("Book"),
              extensions: [],
              properties: [
                PYProperty(
                  doc: None,
                  name: PYKey("title"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                PYProperty(
                  doc: None,
                  name: PYKey("author"),
                  descriptor: Reference(PYReference(
                    identifier: PYIdentifier("Author"),
                    forward: true,
                  )),
                  required: true,
                ),
              ],
              references: [
                PYIdentifier("Author"),
              ],
            )),
            Class(PYClass(
              doc: None,
              name: PYIdentifier("Order"),
              extensions: [],
              properties: [
                PYProperty(
                  doc: None,
                  name: PYKey("book"),
                  descriptor: Reference(PYReference(
                    identifier: PYIdentifier("Book"),
                    forward: false,
                  )),
                  required: true,
                ),
              ],
              references: [
                PYIdentifier("Book"),
              ],
            )),
            Alias(PYAlias(
              doc: None,
              name: PYIdentifier("Name"),
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
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default(),
            ),
            @r#"
        PYConvertModule(PYModule(
          doc: Some(PYDoc("Hello, world!")),
          imports: [],
          definitions: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_reorder() {
        assert_ron_snapshot!(
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Message".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Message".into()),
                            descriptor: GTUnion {
                                span: (0, 0).into(),
                                descriptors: vec![
                                    GtFactory::reference("DM").into(),
                                    GtFactory::reference("Comment").into(),
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "DM".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "DM".into()),
                            descriptor: GTObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((0, 0).into(), "DM".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "message".into()),
                                    descriptor: GtFactory::primitive_string().into(),
                                    required: true,
                                }],
                            }
                            .into(),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Comment".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Comment".into()),
                            descriptor: GTObject {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((0, 0).into(), "Comment".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "message".into()),
                                    descriptor: GtFactory::primitive_string().into(),
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
        PYConvertModule(PYModule(
          doc: None,
          imports: [
            PYImport(
              dependency: Runtime,
              reference: Named([
                Name(PYIdentifier("Model")),
              ]),
            ),
          ],
          definitions: [
            Class(PYClass(
              doc: None,
              name: PYIdentifier("DM"),
              extensions: [],
              properties: [
                PYProperty(
                  doc: None,
                  name: PYKey("message"),
                  descriptor: Primitive(String),
                  required: true,
                ),
              ],
              references: [],
            )),
            Class(PYClass(
              doc: None,
              name: PYIdentifier("Comment"),
              extensions: [],
              properties: [
                PYProperty(
                  doc: None,
                  name: PYKey("message"),
                  descriptor: Primitive(String),
                  required: true,
                ),
              ],
              references: [],
            )),
            Alias(PYAlias(
              doc: None,
              name: PYIdentifier("Message"),
              descriptor: Union(PYUnion(
                descriptors: [
                  Reference(PYReference(
                    identifier: PYIdentifier("DM"),
                    forward: false,
                  )),
                  Reference(PYReference(
                    identifier: PYIdentifier("Comment"),
                    forward: false,
                  )),
                ],
                discriminator: None,
              )),
              references: [
                PYIdentifier("DM"),
                PYIdentifier("Comment"),
              ],
            )),
          ],
        ))
        "#
        );
    }
}
