use genotype_config::GTConfig;
use genotype_lang_py_tree::{py_indent, PYDefinition, PYRender};
use std::path::PathBuf;

use genotype_lang_core_project::{
    module::{GTLangProjectModule, GTLangProjectModuleRender},
    project::{GTLangProject, GTLangProjectRender},
};
use genotype_project::project::GTProject;

use crate::module::PYProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProject {
    pub modules: Vec<PYProjectModule>,
}

impl GTLangProject for PYProject {
    fn generate(
        project: &GTProject,
        config: &GTConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let modules = project
            .modules
            .iter()
            .map(|module| PYProjectModule::generate(&project, module, config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules })
    }

    fn render(&self, config: &GTConfig) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
        let (imports, exports) = self
            .modules
            .iter()
            .fold((vec![], vec![]), |mut acc, module| {
                acc.0
                    .push(format!("from .{} import *", module.name.clone()));

                for definition in module.module.definitions.iter() {
                    acc.1.push(format!(
                        "\"{}\"",
                        match definition {
                            PYDefinition::Class(class) => class.name.0.clone(),
                            PYDefinition::Alias(alias) => alias.name.0.clone(),
                        }
                    ));
                }
                acc
            });
        let barell = GTLangProjectModuleRender {
            path: config.root.join("__init__.py"),
            code: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let mut modules = vec![barell];

        let project_modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectModuleRender {
                path: module.path.clone(),
                code: module.module.render(&py_indent(), config),
            })
            .collect::<Vec<_>>();
        modules.extend(project_modules);

        Ok(GTLangProjectRender { modules })
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
        let config = GTConfig::from_root("./examples/basic");
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
                        path: "./out/author.py".into(),
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
                        name: "book".into(),
                        path: "./out/book.py".into(),
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
        let config = GTConfig::from_root("./examples/glob");
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
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
                        name: "book".into(),
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
        let config = GTConfig::from_root("./examples/basic");
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &config)
                .unwrap()
                .render(&config)
                .unwrap(),
            GTLangProjectRender {
                modules: vec![
                    GTLangProjectModuleRender {
                        path: "./out/__init__.py".into(),
                        code: r#"from .author import *
from .book import *


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GTLangProjectModuleRender {
                        path: "./out/author.py".into(),
                        code: r#"from genotype import Model

class Author(Model):
    name: str
"#
                        .into()
                    },
                    GTLangProjectModuleRender {
                        path: "./out/book.py".into(),
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
