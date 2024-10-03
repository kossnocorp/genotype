use std::path::PathBuf;

use genotype_lang_core_project::{module::GTProjectModuleOut, project::GTProjectOut};
use genotype_project::project::GTProject;

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub out: PathBuf,
    pub modules: Vec<TSProjectModule>,
}

impl GTProjectOut for TSProject {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let out = project.root.join(out);

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
    use std::{collections::HashMap, path::PathBuf, sync::Arc};

    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_project::*;

    #[test]
    fn test_convert_base() {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let author_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/author.type"),
        )
        .unwrap();
        let book_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/book.type"),
        )
        .unwrap();

        assert_eq!(
            TSProject::generate(
                &GTProject {
                    root: root.clone(),
                    modules: vec![
                        GTProjectModule {
                            path: author_path.clone(),
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
                            resolve: GTProjectModuleResolve {
                                deps: HashMap::new(),
                                references: HashMap::new(),
                            },
                        },
                        GTProjectModule {
                            path: book_path.clone(),
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
                            resolve: GTProjectModuleResolve {
                                deps: HashMap::new(),
                                references: HashMap::new(),
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
