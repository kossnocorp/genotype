use genotype_lang_py_config::PYProjectConfig;
use genotype_lang_py_tree::{py_indent, PYDefinition, PYRender};

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

impl GTLangProject<PYProjectConfig> for PYProject {
    fn generate(
        project: &GTProject,
        config: &PYProjectConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let modules = project
            .modules
            .iter()
            .map(|module| PYProjectModule::generate(&project, module, config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules })
    }

    fn render(
        &self,
        config: &PYProjectConfig,
    ) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
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
        let init = GTLangProjectModuleRender {
            path: config.source_path("__init__.py".into()),
            code: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let mut modules = vec![init];

        let project_modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectModuleRender {
                path: module.path.clone(),
                code: module.module.render(&py_indent(), &config.lang),
            })
            .collect::<Vec<_>>();
        modules.extend(project_modules);

        Ok(GTLangProjectRender { modules })
    }
}

#[cfg(test)]
mod tests {
    use genotype_config::GTConfig;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &py_config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
                        path: "py/module/author.py".into(),
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
                        path: "py/module/book.py".into(),
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
        let config = GTConfig::from_root("module", "./examples/glob");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &py_config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
                        path: "py/module/author.py".into(),
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
                        path: "py/module/book.py".into(),
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
        let config = GTConfig::from_root("module", "./examples/basic");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &py_config)
                .unwrap()
                .render(&py_config)
                .unwrap(),
            GTLangProjectRender {
                modules: vec![
                    GTLangProjectModuleRender {
                        path: "py/module/__init__.py".into(),
                        code: r#"from .author import *
from .book import *


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GTLangProjectModuleRender {
                        path: "py/module/author.py".into(),
                        code: r#"from genotype import Model

class Author(Model):
    name: str
"#
                        .into()
                    },
                    GTLangProjectModuleRender {
                        path: "py/module/book.py".into(),
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