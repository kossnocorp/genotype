use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TsProject<'a> {
    pub modules: Vec<TSProjectModule>,
    config: GtConfigPkg<'a, TsConfig>,
}

impl<'a> GtlProject<'a> for TsProject<'a> {
    type Module = TSProjectModule;

    type LangConfig = TsConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.config.pkg_config_ts();
        let modules = project
            .modules
            .iter()
            .map(|module| TSProjectModule::generate(&config, module))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, config })
    }

    fn out(&self) -> Result<GtlProjectOut> {
        let gitignore = GtlProjectFile {
            path: self.config.pkg_file_path(".gitignore".into()),
            source: r#"node_modules"#.into(),
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
                        .strip_prefix(self.project.config.ts.src_path())
                        // [TODO]
                        .unwrap()
                        .as_os_str()
                        .to_str()
                        // [TODO]
                        .unwrap()
                )
            })
            .collect::<Vec<_>>();

        let barrel = GtlProjectFile {
            path: self.config.pkg_src_file_path("index.ts"),
            source: exports.join(""),
        };

        let package_json =
            TsProjectManifest::manifest_file_with_edits(&self.config, &vec![], |doc| {
                doc.insert(
                    "types",
                    self.config
                        .pkg_relative_src_file_path("index.ts")
                        .as_str()
                        .into(),
                );
            })?;

        let project_modules = self
            .modules
            .iter()
            .map(|module| GtlProjectFile {
                path: module.path.clone(),
                source: module
                    .module
                    .render(Default::default(), &mut Default::default())
                    .unwrap(),
            })
            .collect::<Vec<_>>();

        let mut modules = vec![gitignore, package_json, barrel];
        modules.extend(project_modules);

        Ok(GtlProjectOut { files: modules })
    }

    fn modules(&self) -> Vec<Self::Module> {
        self.modules.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_config::GtConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().modules,
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
        let config = GtConfig::from_root("module", "./examples/glob");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().modules,
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
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().out().unwrap(),
            GtlProjectOut {
                files: vec![
                    GtlProjectFile {
                        path: "libs/ts/.gitignore".into(),
                        source: "node_modules".into(),
                    },
                    GtlProjectFile {
                        path: "libs/ts/package.json".into(),
                        source: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/ts/src/index.ts".into(),
                        source: r#"export * from "./author.ts";
export * from "./book.ts";
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/ts/src/author.ts".into(),
                        source: r#"export interface Author {
  name: string;
}
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/ts/src/book.ts".into(),
                        source: r#"import { Author } from "./author.ts";

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
        let mut config = GtConfig::from_root("module", "./examples/dependencies");
        config.ts.common.dependencies = HashMap::from_iter(vec![(
            "genotype_json_types".into(),
            "@genotype/json".into(),
        )]);
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().out().unwrap(),
            GtlProjectOut {
                files: vec![
                    GtlProjectFile {
                        path: "libs/ts/.gitignore".into(),
                        source: "node_modules".into(),
                    },
                    GtlProjectFile {
                        path: "libs/ts/package.json".into(),
                        source: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/ts/src/index.ts".into(),
                        source: r#"export * from "./prompt.ts";
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/ts/src/prompt.ts".into(),
                        source: r#"import { JsonAny } from "@genotype/json";

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
