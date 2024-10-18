use std::sync::{Arc, Mutex};

use genotype_lang_py_tree::module::PYModule;
use genotype_parser::tree::module::GTModule;

use crate::{context::PYConvertContext, convert::PYConvert, resolve::PYConvertResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct PYConvertModule(pub PYModule);

impl PYConvertModule {
    pub fn convert(module: &GTModule, resolve: &PYConvertResolve) -> Self {
        let mut import_context = PYConvertContext::new(resolve.clone(), Box::new(|_| {}));
        let imports = module
            .imports
            .iter()
            .map(|import| import.convert(&mut import_context))
            .collect();

        let definitions = Mutex::new(Vec::new());

        for alias in &module.aliases {
            let hoisted = Arc::new(Mutex::new(Vec::new()));

            let mut context = {
                let hoisted = Arc::clone(&hoisted);
                PYConvertContext::new(
                    resolve.clone(),
                    Box::new(move |definition| {
                        let mut hoisted = hoisted.lock().unwrap();
                        hoisted.push(definition);
                    }),
                )
            };

            // [TODO] Switch to using context to store hoisted definitions
            context.pop_hoisted();

            let definition = alias.convert(&mut context);

            let mut definitions = definitions.lock().unwrap();
            definitions.push(definition);
            definitions.extend(hoisted.lock().unwrap().clone());
        }

        PYConvertModule(PYModule {
            doc: None,
            imports,
            definitions: definitions.into_inner().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::{tree::*, GTSourceCode};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut resolve = PYConvertResolve::new();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_eq!(
            PYConvertModule::convert(
                &GTModule {
                    source_code: GTSourceCode::new("module.type".into(), "".into()),
                    doc: None,
                    imports: vec![
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), ".path.to.module").unwrap(),
                            reference: GTImportReference::Glob((0, 0).into())
                        },
                        GTImport {
                            span: (0, 0).into(),
                            path: GTPath::parse((0, 0).into(), ".path.to.module").unwrap(),
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
                            name: GTIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                name: GTIdentifier::new((0, 0).into(), "User".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
                                        name: GTKey::new((0, 0).into(), "name".into()),
                                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (0, 0).into(),
                                        doc: None,
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
                            name: GTIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (0, 0).into(),
                                name: GTIdentifier::new((0, 0).into(), "Order".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (0, 0).into(),
                                    doc: None,
                                    name: GTKey::new((0, 0).into(), "book".into()),
                                    descriptor: GTDescriptor::Alias(Box::new(GTAlias {
                                        span: (0, 0).into(),
                                        doc: None,
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
                                                    name: GTKey::new((0, 0).into(), "title".into()),
                                                    descriptor: GTDescriptor::Primitive(
                                                        GTPrimitive::String((0, 0).into())
                                                    ),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (0, 0).into(),
                                                    doc: None,
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
                            name: GTIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                        },
                    ],
                },
                &resolve
            ),
            PYConvertModule(PYModule {
                doc: None,
                imports: vec![
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Glob
                    },
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Named(vec![
                            PYImportName::Name("Name".into()),
                            PYImportName::Alias("Name".into(), "Alias".into())
                        ])
                    }
                ],
                definitions: vec![
                    PYDefinition::Class(PYClass {
                        name: "User".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                name: "name".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true,
                            },
                            PYProperty {
                                name: "age".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                                required: false,
                            }
                        ]
                    }),
                    PYDefinition::Class(PYClass {
                        name: "Order".into(),
                        extensions: vec![],
                        properties: vec![PYProperty {
                            name: "book".into(),
                            descriptor: PYReference::new("Book".into(), true).into(),
                            required: true,
                        }]
                    }),
                    PYDefinition::Class(PYClass {
                        name: "Book".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                name: "title".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true,
                            },
                            PYProperty {
                                name: "author".into(),
                                descriptor: PYReference::new("Author".into(), true).into(),
                                required: true,
                            }
                        ]
                    }),
                    PYDefinition::Alias(PYAlias {
                        name: "Name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    }),
                ]
            })
        );
    }
}
