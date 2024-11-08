use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_tree::module::RSModule;
use genotype_parser::tree::module::GTModule;

use crate::{context::RSConvertContext, convert::RSConvert, resolve::RSConvertResolve};

mod ordering;

#[derive(Debug, PartialEq, Clone)]
pub struct RSConvertModule(pub RSModule);

impl RSConvertModule {
    pub fn convert(module: &GTModule, resolve: &RSConvertResolve, config: &RSLangConfig) -> Self {
        // [TODO] Get rid of unnecessary clone
        let mut context = RSConvertContext::new(resolve.clone(), config.clone());

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

        let module = RSModule {
            doc,
            imports,
            definitions,
        };

        // let mut visitor = RSModuleVisitor::new(&module);
        // module.traverse(&mut visitor);

        RSConvertModule(module)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    use super::*;

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
                                                    descriptor: GTIdentifier::new(
                                                        (0, 0).into(),
                                                        "Author".into()
                                                    )
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
            RSConvertModule(RSModule {
                doc: None,
                imports: vec![
                    RSImport {
                        path: ".path.to.module".into(),
                        reference: RSImportReference::Default(Some("module".into())),
                        dependency: RSDependency::Local(".path.to.module".into()),
                    },
                    RSImport {
                        path: ".path.to.module".into(),
                        reference: RSImportReference::Named(vec![
                            RSImportName::Name("Name".into()),
                            RSImportName::Alias("Name".into(), "Alias".into())
                        ]),
                        dependency: RSDependency::Local(".path.to.module".into()),
                    },
                    RSImport {
                        path: "typing".into(),
                        reference: RSImportReference::Named(vec![RSImportName::Name(
                            "Optional".into()
                        )]),
                        dependency: RSDependency::Typing,
                    },
                    RSImport {
                        path: "genotype".into(),
                        reference: RSImportReference::Named(vec![RSImportName::Name(
                            "Model".into()
                        )]),
                        dependency: RSDependency::Runtime,
                    }
                ],
                definitions: vec![
                    RSDefinition::Class(RSClass {
                        doc: None,
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            RSProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                required: true,
                            },
                            RSProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                                required: false,
                            }
                        ],
                        references: vec![],
                    }),
                    RSDefinition::Class(RSClass {
                        doc: None,
                        name: "Book".into(),
                        extensions: vec![],
                        properties: vec![
                            RSProperty {
                                doc: None,
                                name: "title".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                required: true,
                            },
                            RSProperty {
                                doc: None,
                                name: "author".into(),
                                descriptor: RSReference::new("Author".into()).into(),
                                required: true,
                            }
                        ],
                        references: vec![RSIdentifier("Author".into()),],
                    }),
                    RSDefinition::Class(RSClass {
                        doc: None,
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![RSProperty {
                            doc: None,
                            name: "book".into(),
                            descriptor: RSReference::new("Book".into()).into(),
                            required: true,
                        }],
                        references: vec![RSIdentifier("Book".into()),],
                    }),
                    RSDefinition::Alias(RSAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                        references: vec![],
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
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default()
            ),
            RSConvertModule(RSModule {
                doc: Some(RSDoc("Hello, world!".into())),
                imports: vec![],
                definitions: vec![]
            })
        );
    }

    #[test]
    fn test_convert_reorder() {
        assert_eq!(
            RSConvertModule::convert(
                &GTModule {
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: None,
                    imports: vec![],
                    aliases: vec![
                        GTAlias {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 0).into(), "Message".into()),
                            descriptor: GTUnion {
                                span: (0, 0).into(),
                                descriptors: vec![
                                    GTReference(
                                        (0, 0).into(),
                                        GTIdentifier((0, 0).into(), "DM".into())
                                    )
                                    .into(),
                                    GTReference(
                                        (0, 0).into(),
                                        GTIdentifier((0, 0).into(), "Comment".into())
                                    )
                                    .into(),
                                ],
                            }
                            .into(),
                        },
                        GTAlias {
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
            RSConvertModule(RSModule {
                doc: None,
                imports: vec![RSImport {
                    path: "genotype".into(),
                    reference: RSImportReference::Named(vec![RSImportName::Name("Model".into())]),
                    dependency: RSDependency::Runtime,
                }],
                definitions: vec![
                    RSDefinition::Class(RSClass {
                        doc: None,
                        name: "DM".into(),
                        extensions: vec![],
                        properties: vec![RSProperty {
                            doc: None,
                            name: "message".into(),
                            descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            required: true,
                        }],
                        references: vec![],
                    }),
                    RSDefinition::Class(RSClass {
                        doc: None,
                        name: "Comment".into(),
                        extensions: vec![],
                        properties: vec![RSProperty {
                            doc: None,
                            name: "message".into(),
                            descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            required: true,
                        }],
                        references: vec![],
                    }),
                    RSDefinition::Alias(RSAlias {
                        doc: None,
                        name: "Message".into(),
                        descriptor: RSUnion {
                            descriptors: vec![
                                RSReference::new("DM".into()).into(),
                                RSReference::new("Comment".into()).into()
                            ],
                            discriminator: None,
                        }
                        .into(),
                        references: vec![RSIdentifier("DM".into()), RSIdentifier("Comment".into()),],
                    }),
                ]
            })
        );
    }
}
