use genotype_lang_core_project::{
    module::GTProjectModuleOut, path::GTProjectOutPath, project::GTProjectOut,
};
use genotype_project::project::GTProject;

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub out: GTProjectOutPath,
    pub modules: Vec<TSProjectModule>,
}

impl GTProjectOut for TSProject {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let out = GTProjectOutPath::new(&project.root.as_path().join(out));

        let modules = project
            .modules
            .iter()
            .map(|module| TSProjectModule::generate(&project.root, module, &out))
            .collect::<Result<_, _>>()?;

        Ok(Self { out, modules })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_project::*;

    #[test]
    fn test_convert_base() {
        let root: GTProjectPath = "./examples/basic".try_into().unwrap();
        assert_eq!(
            TSProject::generate(
                &GTProject {
                    root: root.clone(),
                    modules: vec![
                        GTProjectModule {
                            path: "./examples/basic/author.type".try_into().unwrap(),
                            deps: vec![],
                            exports: vec![],
                            module: GTModule {
                                doc: None,
                                imports: vec![],
                                aliases: vec![GTAlias {
                                    doc: None,
                                    name: "Author".into(),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        properties: vec![GTProperty {
                                            doc: None,
                                            name: "name".into(),
                                            descriptor: GTDescriptor::Primitive(
                                                GTPrimitive::String
                                            ),
                                            required: true,
                                        }],
                                    }),
                                }],
                            },
                        },
                        GTProjectModule {
                            path: "./examples/basic/book.type".try_into().unwrap(),
                            deps: vec![],
                            exports: vec![],
                            module: GTModule {
                                doc: None,
                                imports: vec![GTImport {
                                    path: "./author".into(),
                                    reference: GTImportReference::Name("Author".into()),
                                }],
                                aliases: vec![GTAlias {
                                    doc: None,
                                    name: "Book".into(),
                                    descriptor: GTDescriptor::Object(GTObject {
                                        properties: vec![
                                            GTProperty {
                                                doc: None,
                                                name: "title".into(),
                                                descriptor: GTDescriptor::Primitive(
                                                    GTPrimitive::String
                                                ),
                                                required: true,
                                            },
                                            GTProperty {
                                                doc: None,
                                                name: "author".into(),
                                                descriptor: GTDescriptor::Reference(
                                                    "Author".into()
                                                ),
                                                required: true,
                                            },
                                        ],
                                    }),
                                }],
                            },
                        },
                    ]
                },
                "out"
            )
            .unwrap(),
            TSProject {
                out: root.as_path().join("out").into(),
                modules: vec![
                    TSProjectModule {
                        path: root.as_path().join("out/author.ts").into(),
                        module: TSModule {
                            doc: None,
                            imports: vec![],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: "Author".into(),
                                properties: vec![TSProperty {
                                    name: "name".into(),
                                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                    required: true,
                                }],
                            })]
                        },
                    },
                    TSProjectModule {
                        path: root.as_path().join("out/book.ts").into(),
                        module: TSModule {
                            doc: None,
                            imports: vec![TSImport {
                                path: "./author".into(),
                                reference: TSImportReference::Named(vec![TSImportName::Name(
                                    "Author".into()
                                )]),
                            }],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: "Book".into(),
                                properties: vec![
                                    TSProperty {
                                        name: "title".into(),
                                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                        required: true,
                                    },
                                    TSProperty {
                                        name: "author".into(),
                                        descriptor: TSDescriptor::Reference("Author".into()),
                                        required: true,
                                    },
                                ],
                            })],
                        },
                    },
                ]
            },
        )
    }
}
