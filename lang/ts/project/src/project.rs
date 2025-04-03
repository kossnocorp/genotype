use std::path::PathBuf;

use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
    source::GTLangProjectSource,
};
use genotype_lang_core_tree::render::GTRender;
use genotype_lang_ts_config::TSProjectConfig;
use genotype_lang_ts_tree::ts_indent;
use genotype_project::project::GTProject;
use miette::Result;

use crate::{module::TSProjectModule, package::TSPackage};

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub modules: Vec<TSProjectModule>,
    config: TSProjectConfig,
}

impl GTLangProject<TSProjectConfig> for TSProject {
    fn generate(project: &GTProject, config: TSProjectConfig) -> Result<Self> {
        let modules = project
            .modules
            .iter()
            .map(|module| TSProjectModule::generate(&project, module, &config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, config })
    }

    fn render(&self) -> Result<GTLangProjectRender> {
        let gitignore = GTLangProjectSource {
            path: self.config.package_path(".gitignore".into()),
            code: r#"node_modules"#.into(),
        };

        let exports = self
            .modules
            .iter()
            .map(|module| {
                format!(
                    r#"export * from "./{}";
"#,
                    module
                        .path
                        .strip_prefix(self.config.out.join(self.config.src.clone()))
                        // [TODO]
                        .unwrap()
                        .as_os_str()
                        .to_str()
                        // [TODO]
                        .unwrap()
                )
            })
            .collect::<Vec<_>>();

        let barrel = GTLangProjectSource {
            path: self.config.source_path("index.ts".into()),
            code: exports.join(""),
        };

        let package = GTLangProjectSource {
            path: self.config.package_path("package.json".into()),
            code: serde_json::to_string_pretty(&TSPackage {
                types: PathBuf::from(self.config.src.clone())
                    .join("index.ts")
                    .as_os_str()
                    .to_str()
                    // [TODO]
                    .unwrap()
                    .into(),
                // [TODO] Merge with package?
                // files: vec![config
                //     .src
                //     .as_os_str()
                //     .to_str()
                //     // [TODO]
                //     .unwrap()
                //     .into()],
                package: self.config.package.clone(),
            })
            .unwrap(),
        };
        let project_modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectSource {
                path: module.path.clone(),
                code: module.module.render(&ts_indent()),
            })
            .collect::<Vec<_>>();

        let mut modules = vec![gitignore, package, barrel];
        modules.extend(project_modules);

        Ok(GTLangProjectRender { files: modules })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use genotype_config::GTConfig;
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, config.as_ts_project())
                .unwrap()
                .modules,
            vec![
                TSProjectModule {
                    path: "libs/ts/src/author.ts".into(),
                    module: TSModule {
                        doc: None,
                        imports: vec![],
                        definitions: vec![TSDefinition::Interface(TSInterface {
                            doc: None,

                            name: "Author".into(),
                            extensions: vec![],
                            properties: vec![TSProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                required: true,
                            }],
                        })]
                    },
                },
                TSProjectModule {
                    path: "libs/ts/src/book.ts".into(),
                    module: TSModule {
                        doc: None,
                        imports: vec![TSImport {
                            path: "./author.ts".into(),
                            reference: TSImportReference::Named(vec![TSImportName::Name(
                                "Author".into()
                            )]),
                        }],
                        definitions: vec![TSDefinition::Interface(TSInterface {
                            doc: None,
                            name: "Book".into(),
                            extensions: vec![],
                            properties: vec![
                                TSProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                    required: true,
                                },
                                TSProperty {
                                    doc: None,
                                    name: "author".into(),
                                    descriptor: TSDescriptor::Reference("Author".into()),
                                    required: true,
                                },
                            ],
                        })],
                    },
                },
            ]
        )
    }

    #[test]
    fn test_convert_glob() {
        let config = GTConfig::from_root("module", "./examples/glob");
        let ts_config = config.as_ts_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, ts_config).unwrap().modules,
            vec![
                TSProjectModule {
                    path: "libs/ts/src/author.ts".into(),
                    module: TSModule {
                        doc: None,
                        imports: vec![],
                        definitions: vec![
                            TSDefinition::Interface(TSInterface {
                                doc: None,
                                name: "Author".into(),
                                extensions: vec![],
                                properties: vec![TSProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: TSDescriptor::Reference("AuthorName".into()),
                                    required: true,
                                }],
                            }),
                            TSDefinition::Alias(TSAlias {
                                doc: None,
                                name: "AuthorName".into(),
                                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                            })
                        ]
                    },
                },
                TSProjectModule {
                    path: "libs/ts/src/book.ts".into(),
                    module: TSModule {
                        doc: None,
                        imports: vec![TSImport {
                            path: "./author.ts".into(),
                            reference: TSImportReference::Glob("author".into()),
                        }],
                        definitions: vec![TSDefinition::Interface(TSInterface {
                            doc: None,
                            name: "Book".into(),
                            extensions: vec![],
                            properties: vec![
                                TSProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                    required: true,
                                },
                                TSProperty {
                                    doc: None,
                                    name: "author".into(),
                                    descriptor: TSDescriptor::Reference("author.Author".into()),
                                    required: true,
                                },
                                TSProperty {
                                    doc: None,
                                    name: "authorName".into(),
                                    descriptor: TSDescriptor::Reference("author.AuthorName".into()),
                                    required: true,
                                },
                            ],
                        })],
                    },
                },
            ]
        )
    }

    #[test]
    fn test_render() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let ts_config = config.as_ts_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            TSProject::generate(&project, ts_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/ts/.gitignore".into(),
                        code: "node_modules".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/ts/package.json".into(),
                        code: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/ts/src/index.ts".into(),
                        code: r#"export * from "./author.ts";
export * from "./book.ts";
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/ts/src/author.ts".into(),
                        code: r#"export interface Author {
  name: string;
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/ts/src/book.ts".into(),
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

    #[test]
    fn test_render_dependencies() {
        let config = GTConfig::from_root("module", "./examples/dependencies");
        let mut ts_config = config.as_ts_project();
        let project = GTProject::load(&config).unwrap();

        ts_config.dependencies = Some(HashMap::from_iter(vec![(
            "genotype_json_tree".into(),
            "@genotype/json".into(),
        )]));

        assert_eq!(
            TSProject::generate(&project, ts_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/ts/.gitignore".into(),
                        code: "node_modules".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/ts/package.json".into(),
                        code: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/ts/src/index.ts".into(),
                        code: r#"export * from "./prompt.ts";
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/ts/src/prompt.ts".into(),
                        code: r#"import { JsonAny } from "@genotype/json";

export interface Prompt {
  content: string;
  output: JsonAny;
}
"#
                        .into()
                    },
                ]
            }
        )
    }
}
