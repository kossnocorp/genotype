use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSConvertModule(pub RSModule);

impl RSConvertModule {
    pub fn convert(
        module: &GTModule,
        resolve: &RSConvertResolve,
        config: &RsConfig,
    ) -> Result<Self> {
        // [TODO] Get rid of unnecessary clone
        let mut context = RSConvertContext::new(
            module.id.clone(),
            resolve.clone(),
            config.lang.clone(),
            config.common.dependencies.clone(),
        );

        let doc = if let Some(doc) = &module.doc {
            let mut doc = doc.convert(&mut context)?;
            doc.1 = true;
            Some(doc)
        } else {
            None
        };

        for import in &module.imports {
            let import = import.convert(&mut context)?;
            context.push_import(import);
        }

        for alias in &module.aliases {
            let definition = alias.convert(&mut context)?;
            context.push_definition(definition);
        }

        let imports = context.drain_imports();

        let definitions = context.drain_definitions();

        let module = RSModule {
            id: module.id.clone(),
            doc,
            imports,
            definitions,
        };

        Ok(RSConvertModule(module))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut resolve = RSConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_ron_snapshot!(
            RSConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::new(
                                (0, 0).into(),
                                GTPathModuleId::Resolved("module/path".into()),
                                "./path/to/module".into()
                            ),
                            reference: GTImportReference::Glob((0, 0).into())
                        },
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::new(
                                (0, 0).into(),
                                GTPathModuleId::Resolved("module/path".into()),
                                "./path/to/module".into()
                            ),
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
                            id: GTDefinitionId("module".into(), "User".into()),
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
                            id: GTDefinitionId("module".into(), "Order".into()),
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
                                }]
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Name".into()),
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
            )
            .unwrap(),
            @r#"
        RSConvertModule(RSModule(
          id: GTModuleId("module"),
          doc: None,
          imports: [
            RSUse(
              dependency: Local(RSPath(GTModuleId("module/path"), "super::path::to::module")),
              reference: Module,
            ),
            RSUse(
              dependency: Local(RSPath(GTModuleId("module/path"), "super::path::to::module")),
              reference: Named([
                Name(RSIdentifier("Name")),
                Alias(RSIdentifier("Name"), RSIdentifier("Alias")),
              ]),
            ),
            RSUse(
              dependency: Serde,
              reference: Named([
                Name(RSIdentifier("Deserialize")),
                Name(RSIdentifier("Serialize")),
              ]),
            ),
          ],
          definitions: [
            Struct(RSStruct(
              id: GTDefinitionId(GTModuleId("module"), "User"),
              doc: None,
              attributes: [
                RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RSIdentifier("User"),
              fields: Resolved([
                RSField(
                  doc: None,
                  attributes: [],
                  name: RSFieldName("name"),
                  descriptor: Primitive(String),
                ),
                RSField(
                  doc: None,
                  attributes: [
                    RSAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
                  ],
                  name: RSFieldName("age"),
                  descriptor: Option(RSOption(
                    descriptor: Primitive(Int32),
                  )),
                ),
              ]),
            )),
            Struct(RSStruct(
              id: GTDefinitionId(GTModuleId("module"), "Order"),
              doc: None,
              attributes: [
                RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RSIdentifier("Order"),
              fields: Resolved([
                RSField(
                  doc: None,
                  attributes: [],
                  name: RSFieldName("book"),
                  descriptor: Reference(RSReference(
                    id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                    identifier: RSIdentifier("Book"),
                    definition_id: GTDefinitionId(GTModuleId("module"), "Book"),
                  )),
                ),
              ]),
            )),
            Struct(RSStruct(
              id: GTDefinitionId(GTModuleId("module"), "Book"),
              doc: None,
              attributes: [
                RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RSIdentifier("Book"),
              fields: Resolved([
                RSField(
                  doc: None,
                  attributes: [],
                  name: RSFieldName("title"),
                  descriptor: Primitive(String),
                ),
                RSField(
                  doc: None,
                  attributes: [],
                  name: RSFieldName("author"),
                  descriptor: Reference(RSReference(
                    id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                    identifier: RSIdentifier("Author"),
                    definition_id: GTDefinitionId(GTModuleId("module"), "Author"),
                  )),
                ),
              ]),
            )),
            Alias(RSAlias(
              id: GTDefinitionId(GTModuleId("module"), "Name"),
              doc: None,
              name: RSIdentifier("Name"),
              descriptor: Primitive(String),
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            RSConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default(),
            )
            .unwrap(),
            @r#"
        RSConvertModule(RSModule(
          id: GTModuleId("module"),
          doc: Some(RSDoc("Hello, world!", true)),
          imports: [],
          definitions: [],
        ))
        "#
        );
    }
}
