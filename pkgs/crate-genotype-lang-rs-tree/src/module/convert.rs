use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RsConvertModule(pub RsModule);

impl RsConvertModule {
    pub fn convert(
        module: &GtModule,
        resolve: &RsConvertResolve,
        config: &RsConfig,
    ) -> Result<Self> {
        // [TODO] Get rid of unnecessary clone
        let mut context = RsConvertContext::new(
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

        let module = RsModule {
            id: module.id.clone(),
            doc,
            imports,
            definitions,
        };

        Ok(RsConvertModule(module))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut resolve = RsConvertResolve::default();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
        );
        resolve.reference_definition_ids.insert(
            GtReferenceId("module".into(), (0, 0).into()),
            GtDefinitionId("module".into(), "Author".into()),
        );

        assert_ron_snapshot!(
            RsConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::new(
                                (0, 0).into(),
                                GtPathModuleId::new((0, 0).into(), "module".into()),
                                "./path/to/module".into()
                            ),
                            reference: GtImportReference::Glob((0, 0).into())
                        },
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::new(
                                (0, 0).into(),
                                GtPathModuleId::new((0, 0).into(), "module".into()),
                                "./path/to/module".into()
                            ),
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
                            id: GtDefinitionId("module".into(), "User".into()),
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
                            id: GtDefinitionId("module".into(), "Order".into()),
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
                                                    descriptor: Gt::reference_anon("Author").into(),
                                                    required: true,
                                                }
                                            ]
                                        })
                                    })),
                                    required: true,
                                }]
                            }),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Name".into()),
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
            )
            .unwrap(),
            @r#"
        RsConvertModule(RsModule(
          id: GtModuleId("module"),
          doc: None,
          imports: [
            RsUse(
              dependency: Local(RsPath(GtModuleId("module/path"), "super::path::to::module")),
              reference: Module,
            ),
            RsUse(
              dependency: Local(RsPath(GtModuleId("module/path"), "super::path::to::module")),
              reference: Named([
                Name(RsIdentifier("Name")),
                Alias(RsIdentifier("Name"), RsIdentifier("Alias")),
              ]),
            ),
            RsUse(
              dependency: Serde,
              reference: Named([
                Name(RsIdentifier("Deserialize")),
                Name(RsIdentifier("Serialize")),
              ]),
            ),
          ],
          definitions: [
            Struct(RsStruct(
              id: GtDefinitionId(GtModuleId("module"), "User"),
              doc: None,
              attributes: [
                RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RsIdentifier("User"),
              fields: Resolved([
                RsField(
                  doc: None,
                  attributes: [],
                  name: RsFieldName("name"),
                  descriptor: Primitive(String),
                ),
                RsField(
                  doc: None,
                  attributes: [
                    RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
                  ],
                  name: RsFieldName("age"),
                  descriptor: Option(RsOption(
                    descriptor: Primitive(Int32),
                  )),
                ),
              ]),
            )),
            Struct(RsStruct(
              id: GtDefinitionId(GtModuleId("module"), "Order"),
              doc: None,
              attributes: [
                RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RsIdentifier("Order"),
              fields: Resolved([
                RsField(
                  doc: None,
                  attributes: [],
                  name: RsFieldName("book"),
                  descriptor: Reference(RsReference(
                    id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                    identifier: RsIdentifier("Book"),
                    definition_id: GtDefinitionId(GtModuleId("module"), "Book"),
                  )),
                ),
              ]),
            )),
            Struct(RsStruct(
              id: GtDefinitionId(GtModuleId("module"), "Book"),
              doc: None,
              attributes: [
                RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              ],
              name: RsIdentifier("Book"),
              fields: Resolved([
                RsField(
                  doc: None,
                  attributes: [],
                  name: RsFieldName("title"),
                  descriptor: Primitive(String),
                ),
                RsField(
                  doc: None,
                  attributes: [],
                  name: RsFieldName("author"),
                  descriptor: Reference(RsReference(
                    id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                    identifier: RsIdentifier("Author"),
                    definition_id: GtDefinitionId(GtModuleId("module"), "Author"),
                  )),
                ),
              ]),
            )),
            Alias(RsAlias(
              id: GtDefinitionId(GtModuleId("module"), "Name"),
              doc: None,
              name: RsIdentifier("Name"),
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
            RsConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default(),
            )
            .unwrap(),
            @r#"
        RsConvertModule(RsModule(
          id: GtModuleId("module"),
          doc: Some(RsDoc("Hello, world!", true)),
          imports: [],
          definitions: [],
        ))
        "#
        );
    }
}
