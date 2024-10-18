use genotype_lang_core_tree::render::GTRender;
use genotype_lang_ts_tree::ts_indent;
use std::path::PathBuf;

use genotype_lang_core_project::{
    module::{GTLangProjectModule, GTLangProjectModuleRender},
    project::{GTLangProject, GTLangProjectRender},
};
use genotype_project::project::GTProject;

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub root: PathBuf,
    pub modules: Vec<TSProjectModule>,
}

impl GTLangProject<()> for TSProject {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root = project.root.join(out);

        let modules = project
            .modules
            .iter()
            .map(|module| TSProjectModule::generate(&project, module, &root))
            .collect::<Result<_, _>>()?;

        Ok(Self { root, modules })
    }

    fn render(&self, _options: &()) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
        let modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectModuleRender {
                path: module.path.clone(),
                code: module.module.render(&ts_indent()),
            })
            .collect::<Vec<_>>();

        Ok(GTLangProjectRender {
            root: self.root.clone(),
            modules,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let project = GTProject::load("./examples/basic", "*.type").unwrap();

        assert_eq!(
            TSProject::generate(&project, "out").unwrap(),
            TSProject {
                root: root.as_path().join("out").into(),
                modules: vec![
                    TSProjectModule {
                        path: root.as_path().join("out/author.ts").into(),
                        module: TSModule {
                            doc: None,
                            imports: vec![],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: "Author".into(),
                                extensions: vec![],
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
                                path: "./author.ts".into(),
                                reference: TSImportReference::Named(vec![TSImportName::Name(
                                    "Author".into()
                                )]),
                            }],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: "Book".into(),
                                extensions: vec![],
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

    #[test]
    fn test_convert_glob() {
        let root = Arc::new(PathBuf::from("./examples/glob").canonicalize().unwrap());
        let project = GTProject::load("./examples/glob", "*.type").unwrap();

        assert_eq!(
            TSProject::generate(&project, "out").unwrap(),
            TSProject {
                root: root.as_path().join("out").into(),
                modules: vec![
                    TSProjectModule {
                        path: root.as_path().join("out/author.ts").into(),
                        module: TSModule {
                            doc: None,
                            imports: vec![],
                            definitions: vec![
                                TSDefinition::Interface(TSInterface {
                                    name: "Author".into(),
                                    extensions: vec![],
                                    properties: vec![TSProperty {
                                        name: "name".into(),
                                        descriptor: TSDescriptor::Reference("AuthorName".into()),
                                        required: true,
                                    }],
                                }),
                                TSDefinition::Alias(TSAlias {
                                    name: "AuthorName".into(),
                                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                })
                            ]
                        },
                    },
                    TSProjectModule {
                        path: root.as_path().join("out/book.ts").into(),
                        module: TSModule {
                            doc: None,
                            imports: vec![TSImport {
                                path: "./author.ts".into(),
                                reference: TSImportReference::Glob("author".into()),
                            }],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: "Book".into(),
                                extensions: vec![],
                                properties: vec![
                                    TSProperty {
                                        name: "title".into(),
                                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                        required: true,
                                    },
                                    TSProperty {
                                        name: "author".into(),
                                        descriptor: TSDescriptor::Reference("author.Author".into()),
                                        required: true,
                                    },
                                    TSProperty {
                                        name: "authorName".into(),
                                        descriptor: TSDescriptor::Reference(
                                            "author.AuthorName".into()
                                        ),
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

    #[test]
    fn test_render() {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let project = GTProject::load("./examples/basic", "*.type").unwrap();

        assert_eq!(
            TSProject::generate(&project, "out")
                .unwrap()
                .render(&())
                .unwrap(),
            GTLangProjectRender {
                root: root.join("out"),
                modules: vec![
                    GTLangProjectModuleRender {
                        path: root.join("out/author.ts"),
                        code: r#"export interface Author {
  name: string;
}
"#
                        .into()
                    },
                    GTLangProjectModuleRender {
                        path: root.join("out/book.ts"),
                        code: r#"import { Author } from "./author.ts";

export interface Book {
  title: string;
  author: Author;
}
"#
                        .into()
                    }
                ]
            }
        )
    }
}
