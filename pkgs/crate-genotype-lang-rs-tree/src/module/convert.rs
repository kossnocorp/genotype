use crate::prelude::internal::*;

impl RsModule {
    pub fn convert(
        module: &GtModule,
        exports: &[GtIdentifier],
        resolve: &RsConvertResolve,
        config: &RsConfig,
    ) -> Result<Self, Box<dyn GtlError>> {
        match Self::convert_inner(module, exports, resolve, config) {
            Ok(module) => Ok(module),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn convert_inner(
        module: &GtModule,
        exports: &[GtIdentifier],
        resolve: &RsConvertResolve,
        config: &RsConfig,
    ) -> RsConvertResult<Self> {
        // [TODO] Get rid of unnecessary clone
        let mut context = RsConvertContext::new(
            module.id.clone(),
            resolve.clone(),
            config.lang.clone(),
            config.common.dependencies.clone(),
        );

        for export in exports {
            let name = export.convert(&mut context)?;
            context.reserve(name);
        }

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

        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert_array_union() {
        assert_ron_snapshot!(
            convert_aliases(vec![Gt::alias(
                "Values",
                Gt::array(Gt::union(vec_into![
                    Gt::primitive_string(),
                    Gt::primitive_i64(),
                    Gt::primitive_boolean(),
                ])),
            )]),
            @r#"
        RsModule(
          id: GtModuleId("module"),
          doc: None,
          imports: [
            RsUse(
              dependency: Serde,
              reference: Named([
                Name(RsIdentifier("Deserialize")),
                Name(RsIdentifier("Serialize")),
              ]),
            ),
          ],
          definitions: [
            Alias(RsAlias(
              id: GtDefinitionId(GtModuleId("module"), "Values"),
              doc: None,
              name: RsIdentifier("Values"),
              generics: [],
              descriptor: Vec(RsVec(
                descriptor: Reference(RsReference(
                  id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                  identifier: RsIdentifier("ValuesElement"),
                  arguments: [],
                  definition_id: GtDefinitionId(GtModuleId("module"), "ValuesElement"),
                )),
              )),
            )),
            Enum(RsEnum(
              id: GtDefinitionId(GtModuleId("module"), "ValuesElement"),
              doc: None,
              attributes: [
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
                RsAttribute("serde(untagged)"),
              ],
              name: RsIdentifier("ValuesElement"),
              generics: [],
              variants: [
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("String"),
                  descriptor: Some(Descriptor(Primitive(String))),
                ),
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Int"),
                  descriptor: Some(Descriptor(Primitive(Int64))),
                ),
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Boolean"),
                  descriptor: Some(Descriptor(Primitive(Boolean))),
                ),
              ],
            )),
          ],
        )
        "#,
        );
    }

    #[test]
    fn test_explicit_export_wins_over_generated_name() {
        let module = RsModule::convert(
            &GtModule {
                id: "module".into(),
                doc: None,
                imports: vec![],
                aliases: vec![Gt::alias(
                    "Values",
                    Gt::array(Gt::union(vec_into![
                        Gt::primitive_string(),
                        Gt::primitive_i64(),
                    ])),
                )],
            },
            &[
                GtIdentifier::new((0, 0).into(), "Values".into()),
                GtIdentifier::new((0, 0).into(), "ValuesElement".into()),
            ],
            &Default::default(),
            &Default::default(),
        )
        .unwrap();

        assert_eq!(
            module
                .definitions
                .iter()
                .map(|definition| definition.name().0.as_ref())
                .collect::<Vec<_>>(),
            ["Values", "ValuesElement2"]
        );
    }

    #[test]
    fn test_convert_tuple_unions() {
        assert_ron_snapshot!(
            convert_aliases(vec![Gt::alias(
                "Point",
                Gt::tuple(vec_into![
                    Gt::union(vec_into![Gt::primitive_f64(), Gt::primitive_i64()]),
                    Gt::union(vec_into![Gt::primitive_f64(), Gt::primitive_i64()]),
                ]),
            )]),
            @r#"
        RsModule(
          id: GtModuleId("module"),
          doc: None,
          imports: [
            RsUse(
              dependency: Serde,
              reference: Named([
                Name(RsIdentifier("Deserialize")),
                Name(RsIdentifier("Serialize")),
              ]),
            ),
          ],
          definitions: [
            Alias(RsAlias(
              id: GtDefinitionId(GtModuleId("module"), "Point"),
              doc: None,
              name: RsIdentifier("Point"),
              generics: [],
              descriptor: Tuple(RsTuple(
                descriptors: [
                  Reference(RsReference(
                    id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                    identifier: RsIdentifier("PointElement0"),
                    arguments: [],
                    definition_id: GtDefinitionId(GtModuleId("module"), "PointElement0"),
                  )),
                  Reference(RsReference(
                    id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                    identifier: RsIdentifier("PointElement1"),
                    arguments: [],
                    definition_id: GtDefinitionId(GtModuleId("module"), "PointElement1"),
                  )),
                ],
              )),
            )),
            Enum(RsEnum(
              id: GtDefinitionId(GtModuleId("module"), "PointElement0"),
              doc: None,
              attributes: [
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
                RsAttribute("serde(untagged)"),
              ],
              name: RsIdentifier("PointElement0"),
              generics: [],
              variants: [
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Float"),
                  descriptor: Some(Descriptor(Primitive(Float64))),
                ),
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Int"),
                  descriptor: Some(Descriptor(Primitive(Int64))),
                ),
              ],
            )),
            Enum(RsEnum(
              id: GtDefinitionId(GtModuleId("module"), "PointElement1"),
              doc: None,
              attributes: [
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
                RsAttribute("serde(untagged)"),
              ],
              name: RsIdentifier("PointElement1"),
              generics: [],
              variants: [
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Float"),
                  descriptor: Some(Descriptor(Primitive(Float64))),
                ),
                RsEnumVariant(
                  doc: None,
                  attributes: [],
                  name: RsIdentifier("Int"),
                  descriptor: Some(Descriptor(Primitive(Int64))),
                ),
              ],
            )),
          ],
        )
        "#,
        );
    }

    #[test]
    fn test_explicit_export_wins_over_generated_tuple_name() {
        let module = RsModule::convert(
            &GtModule {
                id: "module".into(),
                doc: None,
                imports: vec![],
                aliases: vec![Gt::alias(
                    "Point",
                    Gt::tuple(vec_into![
                        Gt::union(vec_into![Gt::primitive_f64(), Gt::primitive_i64()]),
                        Gt::union(vec_into![Gt::primitive_f64(), Gt::primitive_i64()]),
                    ]),
                )],
            },
            &[
                GtIdentifier::new((0, 0).into(), "Point".into()),
                GtIdentifier::new((0, 0).into(), "PointElement0".into()),
            ],
            &Default::default(),
            &Default::default(),
        )
        .unwrap();

        assert_eq!(
            module
                .definitions
                .iter()
                .map(|definition| definition.name().0.as_ref())
                .collect::<Vec<_>>(),
            ["Point", "PointElement02", "PointElement1"]
        );
    }

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
            RsModule::convert(
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
                            generics: vec![],
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
                            generics: vec![],
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
                                        generics: vec![],
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
                            generics: vec![],
                            descriptor: Gt::primitive_string().into(),
                        },
                    ],
                },
                &[],
                &resolve,
                &Default::default(),
            )
            .unwrap(),
            @r#"
        RsModule(
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
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
              ],
              name: RsIdentifier("User"),
              generics: [],
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
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
              ],
              name: RsIdentifier("Order"),
              generics: [],
              fields: Resolved([
                RsField(
                  doc: None,
                  attributes: [],
                  name: RsFieldName("book"),
                  descriptor: Reference(RsReference(
                    id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                    identifier: RsIdentifier("Book"),
                    arguments: [],
                    definition_id: GtDefinitionId(GtModuleId("module"), "Book"),
                  )),
                ),
              ]),
            )),
            Struct(RsStruct(
              id: GtDefinitionId(GtModuleId("module"), "Book"),
              doc: None,
              attributes: [
                RsAttribute("derive(Clone, Debug, Deserialize, PartialEq, Serialize)"),
              ],
              name: RsIdentifier("Book"),
              generics: [],
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
                    arguments: [],
                    definition_id: GtDefinitionId(GtModuleId("module"), "Author"),
                  )),
                ),
              ]),
            )),
            Alias(RsAlias(
              id: GtDefinitionId(GtModuleId("module"), "Name"),
              doc: None,
              name: RsIdentifier("Name"),
              generics: [],
              descriptor: Primitive(String),
            )),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            RsModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &[],
                &Default::default(),
                &Default::default(),
            )
            .unwrap(),
            @r#"
        RsModule(
          id: GtModuleId("module"),
          doc: Some(RsDoc("Hello, world!", true)),
          imports: [],
          definitions: [],
        )
        "#
        );
    }

    fn convert_aliases(aliases: Vec<GtAlias>) -> RsModule {
        let exports = aliases
            .iter()
            .map(|alias| alias.name.clone())
            .collect::<Vec<_>>();
        RsModule::convert(
            &GtModule {
                id: "module".into(),
                doc: None,
                imports: vec![],
                aliases,
            },
            &exports,
            &Default::default(),
            &Default::default(),
        )
        .unwrap()
    }
}
