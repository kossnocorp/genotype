use crate::prelude::internal::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct RSConvertModule(pub RSModule);

impl RSConvertModule {
    pub fn convert(
        module: &GTModule,
        resolve: &RSConvertResolve,
        config: &RSLangConfig,
        dependecies_config: Option<HashMap<String, String>>,
    ) -> Result<Self> {
        // [TODO] Get rid of unnecessary clone
        let mut context = RSConvertContext::new(
            module.id.clone(),
            resolve.clone(),
            config.clone(),
            dependecies_config,
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let mut resolve = RSConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_eq!(
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
                                name: GTIdentifier::new((0, 0).into(), "User".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((0, 0).into(), "name".into()),
                                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((0, 0).into(), "age".into()),
                                        descriptor: GTPrimitive::Int32((0, 0).into()).into(),
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
                                                        GTPrimitive::String((0, 0).into())
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
                                                    descriptor: GTReference {
                                                        span: (0, 0).into(),
                                                        id: GTReferenceId(
                                                            "module".into(),
                                                            (0, 1).into()
                                                        ),
                                                        definition_id:
                                                            GTReferenceDefinitionId::Resolved(
                                                                GTDefinitionId(
                                                                    "module".into(),
                                                                    "Author".into()
                                                                )
                                                            ),
                                                        identifier: GTIdentifier::new(
                                                            (0, 0).into(),
                                                            "Author".into()
                                                        )
                                                    }
                                                    .into(),
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
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                        },
                    ],
                },
                &resolve,
                &Default::default(),
                None
            )
            .unwrap(),
            RSConvertModule(RSModule {
                id: "module".into(),
                doc: None,
                imports: vec![
                    RSUse {
                        reference: RSUseReference::Module,
                        dependency: RSDependencyIdent::Local(RSPath(
                            GTModuleId("module/path".into()),
                            "super::path::to::module".into()
                        ))
                    },
                    RSUse {
                        reference: RSUseReference::Named(vec![
                            RSUseName::Name("Name".into()),
                            RSUseName::Alias("Name".into(), "Alias".into())
                        ]),
                        dependency: RSDependencyIdent::Local(RSPath(
                            GTModuleId("module/path".into()),
                            "super::path::to::module".into()
                        ))
                    },
                    RSUse {
                        reference: RSUseReference::Named(vec![
                            RSUseName::Name("Deserialize".into(),),
                            RSUseName::Name("Serialize".into())
                        ]),
                        dependency: RSDependencyIdent::Serde,
                    }
                ],
                definitions: vec![
                    RSDefinition::Struct(RSStruct {
                        id: GTDefinitionId("module".into(), "User".into()),
                        doc: None,
                        attributes: vec![
                            "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
                        ],
                        name: "User".into(),
                        fields: vec![
                            RSField {
                                doc: None,
                                attributes: vec![],
                                name: "name".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSField {
                                doc: None,
                                attributes: vec![],
                                name: "age".into(),
                                descriptor: RSOption::new(RSPrimitive::Int32.into()).into(),
                            }
                        ]
                        .into(),
                    }),
                    RSDefinition::Struct(RSStruct {
                        id: GTDefinitionId("module".into(), "Order".into()),
                        doc: None,
                        attributes: vec![
                            "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
                        ],
                        name: "Order".into(),
                        fields: vec![RSField {
                            doc: None,
                            attributes: vec![],
                            name: "book".into(),
                            descriptor: RSReference {
                                id: GTReferenceId("module".into(), (0, 0).into()),
                                identifier: "Book".into(),
                                definition_id: GTDefinitionId("module".into(), "Book".into())
                            }
                            .into(),
                        }]
                        .into(),
                    }),
                    RSDefinition::Struct(RSStruct {
                        id: GTDefinitionId("module".into(), "Book".into()),
                        doc: None,
                        attributes: vec![
                            "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
                        ],
                        name: "Book".into(),
                        fields: vec![
                            RSField {
                                doc: None,
                                attributes: vec![],
                                name: "title".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSField {
                                doc: None,
                                attributes: vec![],
                                name: "author".into(),
                                descriptor: RSReference {
                                    id: GTReferenceId("module".into(), (0, 1).into()),
                                    identifier: "Author".into(),
                                    definition_id: GTDefinitionId("module".into(), "Author".into())
                                }
                                .into(),
                            }
                        ]
                        .into(),
                    }),
                    RSDefinition::Alias(RSAlias {
                        id: GTDefinitionId("module".into(), "Name".into()),
                        doc: None,
                        name: "Name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_eq!(
            RSConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default(),
                None
            )
            .unwrap(),
            RSConvertModule(RSModule {
                id: "module".into(),
                doc: Some(RSDoc::new("Hello, world!", true)),
                imports: vec![],
                definitions: vec![]
            })
        );
    }
}
