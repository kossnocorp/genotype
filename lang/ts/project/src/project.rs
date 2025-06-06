use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TsProject<'a> {
    pub modules: Vec<TsProjectModule>,
    pub config: GtConfigPkg<'a, TsConfig>,
}

impl<'a> GtlProject<'a> for TsProject<'a> {
    type Module = TsProjectModule;

    type LangConfig = TsConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.config.pkg_config_ts();
        let modules = project
            .modules
            .iter()
            .map(|module| TsProjectModule::generate(&config.target, module))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, config })
    }

    fn dist(&self) -> Result<GtlProjectDist> {
        let gitignore = GtlProjectFile {
            path: self.config.pkg_file_path(&".gitignore".into()),
            source: r#"node_modules"#.into(),
        };

        let exports = self
            .modules
            .iter()
            .map(|module| {
                format!(
                    r#"export * from "./{}";
"#,
                    module.path.as_str()
                )
            })
            .collect::<Vec<_>>();

        let barrel = GtlProjectFile {
            path: self.config.pkg_src_file_path(&"index.ts".into()),
            source: exports.join(""),
        };

        let package_json = self.generate_manifest(&vec![])?;

        let project_modules = self
            .modules
            .iter()
            .map(|module| {
                let path = self.config.pkg_src_file_path(&module.path);
                let source = module
                    .module
                    .render(Default::default(), &mut Default::default())
                    .unwrap();
                GtlProjectFile { path, source }
            })
            .collect::<Vec<_>>();

        let mut modules = vec![gitignore, package_json, barrel];
        modules.extend(project_modules);

        Ok(GtlProjectDist { files: modules })
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
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().modules,
            vec![
                TsProjectModule {
                    path: "author.ts".into(),
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
                TsProjectModule {
                    path: "book.ts".into(),
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
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().modules,
            vec![
                TsProjectModule {
                    path: "author.ts".into(),
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
                TsProjectModule {
                    path: "book.ts".into(),
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
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/basic/dist/ts/.gitignore".into(),
                        source: "node_modules".into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/ts/package.json".into(),
                        source: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/ts/src/index.ts".into(),
                        source: r#"export * from "./author.ts";
export * from "./book.ts";
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/ts/src/author.ts".into(),
                        source: r#"export interface Author {
  name: string;
}
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/ts/src/book.ts".into(),
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
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            TsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/dependencies/dist/ts/.gitignore".into(),
                        source: "node_modules".into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/ts/package.json".into(),
                        source: r#"{
  "types": "src/index.ts"
}"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/ts/src/index.ts".into(),
                        source: r#"export * from "./prompt.ts";
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/ts/src/prompt.ts".into(),
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
