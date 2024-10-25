use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_tree::module::PYModule;
use genotype_parser::tree::module::GTModule;

use crate::{context::PYConvertContext, convert::PYConvert, resolve::PYConvertResolve};

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
        let definitions = context.drain_definitions();

        PYConvertModule(PYModule {
            doc,
            imports,
            definitions,
        })
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
                        ]
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![PYProperty {
                            doc: None,
                            name: "book".into(),
                            descriptor: PYReference::new("Book".into(), true).into(),
                            required: true,
                        }]
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
                        ]
                    }),
                    PYDefinition::Alias(PYAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
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
}
