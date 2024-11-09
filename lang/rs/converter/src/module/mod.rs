use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_tree::module::RSModule;
use genotype_parser::tree::module::GTModule;

use crate::{context::RSConvertContext, convert::RSConvert, resolve::RSConvertResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct RSConvertModule(pub RSModule);

impl RSConvertModule {
    pub fn convert(module: &GTModule, resolve: &RSConvertResolve, config: &RSLangConfig) -> Self {
        // [TODO] Get rid of unnecessary clone
        let mut context = RSConvertContext::new(resolve.clone(), config.clone());

        let doc = module.doc.as_ref().map(|doc| {
            let mut doc = doc.convert(&mut context);
            doc.1 = true;
            doc
        });

        for import in &module.imports {
            let import = import.convert(&mut context);
            context.push_import(import);
        }

        for alias in &module.aliases {
            let definition = alias.convert(&mut context);
            context.push_definition(definition);
        }

        let imports = context.drain_imports();

        let definitions = context.drain_definitions();

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
                        path: "self::path::to::module".into(),
                        reference: RSImportReference::Default(Some("module".into())),
                        dependency: RSDependency::Local("self::path::to::module".into()),
                    },
                    RSImport {
                        path: "self::path::to::module".into(),
                        reference: RSImportReference::Named(vec![
                            RSImportName::Name("Name".into()),
                            RSImportName::Alias("Name".into(), "Alias".into())
                        ]),
                        dependency: RSDependency::Local("self::path::to::module".into()),
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
                    RSDefinition::Struct(RSStruct {
                        doc: None,
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "name".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "age".into(),
                                descriptor: RSOption::new(RSPrimitive::Int.into()).into(),
                            }
                        ],
                    }),
                    RSDefinition::Struct(RSStruct {
                        doc: None,
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![RSProperty {
                            doc: None,
                            attributes: vec![],
                            name: "book".into(),
                            descriptor: RSReference::new("Book".into()).into(),
                        }],
                    }),
                    RSDefinition::Struct(RSStruct {
                        doc: None,
                        name: "Book".into(),
                        extensions: vec![],
                        properties: vec![
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "title".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "author".into(),
                                descriptor: RSReference::new("Author".into()).into(),
                            }
                        ],
                    }),
                    RSDefinition::Alias(RSAlias {
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
                    source_code: NamedSource::new("module.type", "".into()),
                    doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                &Default::default(),
                &Default::default()
            ),
            RSConvertModule(RSModule {
                doc: Some(RSDoc::new("Hello, world!", true)),
                imports: vec![],
                definitions: vec![]
            })
        );
    }
}
