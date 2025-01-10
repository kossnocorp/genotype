use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_tree::module::PYModule;
use genotype_lang_py_visitor::traverse::PYTraverse;
use genotype_parser::GTModule;
use visitor::PYModuleVisitor;

use crate::{context::PYConvertContext, convert::PYConvert, resolve::PYConvertResolve};

mod ordering;
mod visitor;

#[derive(Debug, PartialEq, Clone)]
pub struct PYConvertModule(pub PYModule);

impl PYConvertModule {
    pub fn convert(module: &GTModule, resolve: &PYConvertResolve, config: &PYLangConfig) -> Self {
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
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut resolve = PYConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_eq!(
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    source_code: NamedSource::new("module.type", "".into()),
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
                                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
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
                                                            (0, 0).into()
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
                                },]
                            }),
                        },
                        GTAlias {
                            id: GTDefinitionId("module".into(), "Order".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                        },
                    ],
                },
                &resolve,
                &Default::default()
            ),
            PYConvertModule(PYModule {
                doc: None,
                imports: vec![
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Default(Some("module".into())),
                        dependency: PYDependency::Local(".path.to.module".into()),
                    },
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Named(vec![
                            PYImportName::Name("Name".into()),
                            PYImportName::Alias("Name".into(), "Alias".into())
                        ]),
                        dependency: PYDependency::Local(".path.to.module".into()),
                    },
                    PYImport {
                        path: "typing".into(),
                        reference: PYImportReference::Named(vec![PYImportName::Name(
                            "Optional".into()
                        )]),
                        dependency: PYDependency::Typing,
                    },
                    PYImport {
                        path: "genotype".into(),
                        reference: PYImportReference::Named(vec![PYImportName::Name(
                            "Model".into()
                        )]),
                        dependency: PYDependency::Runtime,
                    }
                ],
                definitions: vec![
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true,
                            },
                            PYProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                                required: false,
                            }
                        ],
                        references: vec![],
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Book".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                doc: None,
                                name: "title".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true,
                            },
                            PYProperty {
                                doc: None,
                                name: "author".into(),
                                descriptor: PYReference::new("Author".into(), true).into(),
                                required: true,
                            }
                        ],
                        references: vec![PYIdentifier("Author".into()),],
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![PYProperty {
                            doc: None,
                            name: "book".into(),
                            descriptor: PYReference::new("Book".into(), false).into(),
                            required: true,
                        }],
                        references: vec![PYIdentifier("Book".into()),],
                    }),
                    PYDefinition::Alias(PYAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        references: vec![],
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_eq!(
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default()
            ),
            PYConvertModule(PYModule {
                doc: Some(PYDoc("Hello, world!".into())),
                imports: vec![],
                definitions: vec![]
            })
        );
    }

    #[test]
    fn test_convert_reorder() {
        assert_eq!(
            PYConvertModule::convert(
                &GTModule {
                    id: "module".into(),
                    source_code: NamedSource::new("module.type", "".into()),
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
                                    GTReference {
                                        span: (0, 0).into(),
                                        id: GTReferenceId("module".into(), (0, 0).into()),
                                        definition_id: GTReferenceDefinitionId::Resolved(
                                            GTDefinitionId("module".into(), "DM".into())
                                        ),
                                        identifier: GTIdentifier((0, 0).into(), "DM".into())
                                    }
                                    .into(),
                                    GTReference {
                                        span: (0, 0).into(),
                                        id: GTReferenceId("module".into(), (0, 0).into()),
                                        definition_id: GTReferenceDefinitionId::Resolved(
                                            GTDefinitionId("module".into(), "Comment".into())
                                        ),
                                        identifier: GTIdentifier((0, 0).into(), "Comment".into())
                                    }
                                    .into(),
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
                                name: GTIdentifier::new((0, 0).into(), "DM".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "message".into()),
                                    descriptor: GTPrimitive::String((0, 0).into()).into(),
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
                                name: GTIdentifier::new((0, 0).into(), "Comment".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((0, 0).into(), "message".into()),
                                    descriptor: GTPrimitive::String((0, 0).into()).into(),
                                    required: true,
                                }],
                            }
                            .into(),
                        }
                    ],
                },
                &Default::default(),
                &Default::default()
            ),
            PYConvertModule(PYModule {
                doc: None,
                imports: vec![PYImport {
                    path: "genotype".into(),
                    reference: PYImportReference::Named(vec![PYImportName::Name("Model".into())]),
                    dependency: PYDependency::Runtime,
                }],
                definitions: vec![
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "DM".into(),
                        extensions: vec![],
                        properties: vec![PYProperty {
                            doc: None,
                            name: "message".into(),
                            descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                            required: true,
                        }],
                        references: vec![],
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Comment".into(),
                        extensions: vec![],
                        properties: vec![PYProperty {
                            doc: None,
                            name: "message".into(),
                            descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                            required: true,
                        }],
                        references: vec![],
                    }),
                    PYDefinition::Alias(PYAlias {
                        doc: None,
                        name: "Message".into(),
                        descriptor: PYUnion {
                            descriptors: vec![
                                PYReference::new("DM".into(), false).into(),
                                PYReference::new("Comment".into(), false).into()
                            ],
                            discriminator: None,
                        }
                        .into(),
                        references: vec![PYIdentifier("DM".into()), PYIdentifier("Comment".into()),],
                    }),
                ]
            })
        );
    }
}
