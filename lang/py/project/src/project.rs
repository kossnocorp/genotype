use genotype_lang_py_tree::{py_indent, PYOptions, PYRender};
use std::path::PathBuf;

use genotype_lang_core_project::{
    module::{GTLangProjectModule, GTLangProjectModuleRender},
    project::{GTLangProject, GTLangProjectRender},
};
use genotype_project::project::GTProject;

use crate::module::PYProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProject {
    pub root: PathBuf,
    pub modules: Vec<PYProjectModule>,
}

impl GTLangProject<PYOptions> for PYProject {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root = project.root.join(out);

        let modules = project
            .modules
            .iter()
            .map(|module| PYProjectModule::generate(&project, module, &root))
            .collect::<Result<_, _>>()?;

        Ok(Self { root, modules })
    }

    fn render(
        &self,
        options: &PYOptions,
    ) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
        let modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectModuleRender {
                path: module.path.clone(),
                code: module.module.render(&py_indent(), options),
            })
            .collect::<Vec<_>>();

        Ok(GTLangProjectRender {
            root: self.root.clone(),
            modules,
        })
    }
}

#[cfg(test)]
mod tespy {
    use std::{path::PathBuf, sync::Arc};

    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let project = GTProject::load("./examples/basic", "*.type").unwrap();

        assert_eq!(
            PYProject::generate(&project, "out").unwrap(),
            PYProject {
                root: root.as_path().join("out").into(),
                modules: vec![
                    PYProjectModule {
                        path: root.as_path().join("out/author.py").into(),
                        module: PYModule {
                            doc: None,
                            imports: vec![PYImport {
                                path: "genotype".into(),
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                            }],
                            definitions: vec![PYDefinition::Class(PYClass {
                                name: "Author".into(),
                                extensions: vec![],
                                properties: vec![PYProperty {
                                    name: "name".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                    required: true,
                                }],
                            })]
                        },
                    },
                    PYProjectModule {
                        path: root.as_path().join("out/book.py").into(),
                        module: PYModule {
                            doc: None,
                            imports: vec![
                                PYImport {
                                    path: ".author".into(),
                                    reference: PYImportReference::Named(vec![PYImportName::Name(
                                        "Author".into()
                                    )]),
                                },
                                PYImport {
                                    path: "genotype".into(),
                                    reference: PYImportReference::Named(vec![PYImportName::Name(
                                        "Model".into()
                                    )]),
                                }
                            ],
                            definitions: vec![PYDefinition::Class(PYClass {
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
                                        descriptor: PYReference::new("Author".into(), false).into(),
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
            PYProject::generate(&project, "out").unwrap(),
            PYProject {
                root: root.as_path().join("out").into(),
                modules: vec![
                    PYProjectModule {
                        path: root.as_path().join("out/author.py").into(),
                        module: PYModule {
                            doc: None,
                            imports: vec![PYImport {
                                path: "genotype".into(),
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                            }],
                            definitions: vec![
                                PYDefinition::Class(PYClass {
                                    name: "Author".into(),
                                    extensions: vec![],
                                    properties: vec![PYProperty {
                                        name: "name".into(),
                                        descriptor: PYReference::new("AuthorName".into(), true)
                                            .into(),
                                        required: true,
                                    }],
                                }),
                                PYDefinition::Alias(PYAlias {
                                    name: "AuthorName".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                })
                            ]
                        },
                    },
                    PYProjectModule {
                        path: root.as_path().join("out/book.py").into(),
                        module: PYModule {
                            doc: None,
                            imports: vec![
                                PYImport {
                                    path: ".author".into(),
                                    reference: PYImportReference::Default(Some("author".into())),
                                },
                                PYImport {
                                    path: "genotype".into(),
                                    reference: PYImportReference::Named(vec![PYImportName::Name(
                                        "Model".into()
                                    )]),
                                }
                            ],
                            definitions: vec![PYDefinition::Class(PYClass {
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
                                        descriptor: PYReference::new("author.Author".into(), false)
                                            .into(),
                                        required: true,
                                    },
                                    PYProperty {
                                        name: "author_name".into(),
                                        descriptor: PYReference::new(
                                            "author.AuthorName".into(),
                                            false
                                        )
                                        .into(),
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
            PYProject::generate(&project, "out")
                .unwrap()
                .render(&PYOptions::default())
                .unwrap(),
            GTLangProjectRender {
                root: root.join("out"),
                modules: vec![
                    GTLangProjectModuleRender {
                        path: root.join("out/author.py"),
                        code: r#"from genotype import Model

class Author(Model):
    name: str
"#
                        .into()
                    },
                    GTLangProjectModuleRender {
                        path: root.join("out/book.py"),
                        code: r#"from .author import Author
from genotype import Model

class Book(Model):
    title: str
    author: Author
"#
                        .into()
                    }
                ]
            }
        )
    }
}
