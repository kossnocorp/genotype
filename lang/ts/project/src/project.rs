use genotype_lang_core_tree::render::GTRender;
use genotype_lang_ts_config::TSProjectConfig;
use genotype_lang_ts_tree::ts_indent;

use genotype_lang_core_project::{
    module::{GTLangProjectModule, GTLangProjectModuleRender},
    project::{GTLangProject, GTLangProjectRender},
};
use genotype_project::project::GTProject;

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub modules: Vec<TSProjectModule>,
}

impl GTLangProject<TSProjectConfig> for TSProject {
    fn generate(
        project: &GTProject,
        config: &TSProjectConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let modules = project
            .modules
            .iter()
            .map(|module| TSProjectModule::generate(&project, module, config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules })
    }

    fn render(
        &self,
        _config: &TSProjectConfig,
    ) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
        let modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectModuleRender {
                path: module.path.clone(),
                code: module.module.render(&ts_indent()),
            })
            .collect::<Vec<_>>();

        Ok(GTLangProjectRender { modules })
    }
}

#[cfg(test)]
mod tests {
    use genotype_config::GTConfig;
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, &config.as_ts_project()).unwrap(),
            TSProject {
                modules: vec![
                    TSProjectModule {
                        path: "ts/src/author.ts".into(),
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
                        path: "ts/src/book.ts".into(),
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
        let config = GTConfig::from_root("module", "./examples/glob");
        let ts_config = config.as_ts_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, &ts_config).unwrap(),
            TSProject {
                modules: vec![
                    TSProjectModule {
                        path: "ts/src/author.ts".into(),
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
                        path: "ts/src/book.ts".into(),
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
        let config = GTConfig::from_root("module", "./examples/basic");
        let ts_config = config.as_ts_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, &ts_config)
                .unwrap()
                .render(&ts_config)
                .unwrap(),
            GTLangProjectRender {
                modules: vec![
                    GTLangProjectModuleRender {
                        path: "ts/src/author.ts".into(),
                        code: r#"export interface Author {
  name: string;
}
"#
                        .into()
                    },
                    GTLangProjectModuleRender {
                        path: "ts/src/book.ts".into(),
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
