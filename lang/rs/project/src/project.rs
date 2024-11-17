use std::path::PathBuf;

use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
    source::GTLangProjectSource,
};
use genotype_lang_rs_config::RSProjectConfig;
use genotype_lang_rs_tree::{rs_indent, RSRender};
use genotype_project::project::GTProject;
use indexmap::{IndexMap, IndexSet};
use miette::Result;

use crate::module::RSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProject {
    pub modules: Vec<RSProjectModule>,
}

impl GTLangProject<RSProjectConfig> for RSProject {
    fn generate(project: &GTProject, config: &RSProjectConfig) -> Result<Self> {
        let modules = project
            .modules
            .iter()
            .map(|module| RSProjectModule::generate(&project, module, config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules })
    }

    fn render(&self, config: &RSProjectConfig) -> Result<GTLangProjectRender> {
        let gitignore = GTLangProjectSource {
            path: config.package_path(".gitignore".into()),
            code: r#"target"#.into(),
        };

        let dependencies = self
            .modules
            .iter()
            .flat_map(|module| {
                module
                    .module
                    .imports
                    .iter()
                    .map(|import| import.dependency.clone())
            })
            .collect::<IndexSet<_>>();

        let dependencies = dependencies
            .iter()
            .fold("[dependencies]".into(), |acc, dependency| {
                if let Some(str) = dependency.external_str() {
                    format!("{acc}\n{str}")
                } else {
                    acc
                }
            });

        let cargo = if let Some(cargo) = &config.package {
            format!("{cargo}\n")
        } else {
            "".into()
        };

        let cargo = GTLangProjectSource {
            path: config.package_path("Cargo.toml".into()),
            code: format!(
                r#"{cargo}

{dependencies}
"#,
            ),
        };

        let src_root = config.src_path();
        let mut module_paths: IndexMap<PathBuf, IndexSet<String>> = IndexMap::new();

        for module in self.modules.iter() {
            let mut module: PathBuf = module.path.clone();
            loop {
                let path: PathBuf = module.parent().unwrap().into();
                let name = module
                    .with_extension("")
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();

                module_paths
                    .entry(path.clone())
                    .and_modify(|paths| {
                        paths.insert(name.clone());
                    })
                    .or_insert_with(|| IndexSet::from_iter(vec![name]));

                if path == src_root {
                    break;
                }

                module = path;
            }
        }

        let module_inits = module_paths.into_iter().map(|(module_path, modules)| {
            let path = module_path.join(if src_root == module_path {
                "lib.rs"
            } else {
                "mod.rs"
            });

            let code = modules
                .iter()
                .map(|module| format!("pub mod {};", module))
                .collect::<Vec<_>>()
                .join("\n");

            GTLangProjectSource { path, code }
        });

        let project_modules = self
            .modules
            .iter()
            .map(
                |module| match module.module.render(&rs_indent(), &config.lang) {
                    Ok(code) => Ok(GTLangProjectSource {
                        path: module.path.clone(),
                        code,
                    }),
                    Err(err) => Err(err),
                },
            )
            .collect::<Result<Vec<_>>>()?;

        let mut files = vec![gitignore, cargo];
        files.extend(module_inits);
        files.extend(project_modules);

        Ok(GTLangProjectRender { files })
    }
}

#[cfg(test)]
mod tests {
    use genotype_config::GTConfig;
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, &rs_config).unwrap(),
            RSProject {
                modules: vec![
                    RSProjectModule {
                        name: "author".into(),
                        path: "rs/src/author.rs".into(),
                        module: RSModule {
                            doc: None,
                            imports: vec![RSUse {
                                path: "serde".into(),
                                reference: RSUseReference::Named(vec![
                                    RSUseName::Name("Deserialize".into()),
                                    RSUseName::Name("Serialize".into())
                                ]),
                                dependency: RSDependency::Serde,
                            }],
                            definitions: vec![RSDefinition::Struct(RSStruct {
                                doc: None,
                                attributes: vec![
                                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                                        .into()
                                ],
                                name: "Author".into(),
                                fields: vec![RSField {
                                    doc: None,
                                    attributes: vec![],
                                    name: "name".into(),
                                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                }]
                                .into(),
                            })]
                        },
                    },
                    RSProjectModule {
                        name: "book".into(),
                        path: "rs/src/book.rs".into(),
                        module: RSModule {
                            doc: None,
                            imports: vec![
                                RSUse {
                                    path: "self::author".into(),
                                    reference: RSUseReference::Named(vec![RSUseName::Name(
                                        "Author".into()
                                    )]),
                                    dependency: RSDependency::Local("self::author".into()),
                                },
                                RSUse {
                                    path: "serde".into(),
                                    reference: RSUseReference::Named(vec![RSUseName::Name(
                                        "Deserialize".into()
                                    ), RSUseName::Name(
                                        "Serialize".into()
                                    )]),
                                    dependency: RSDependency::Serde,
                                }
                            ],
                            definitions: vec![RSDefinition::Struct(RSStruct {
                                doc: None,
                                attributes: vec![
                                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                                        .into()
                                ],
                                name: "Book".into(),
                                fields: vec![
                                    RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "title".into(),
                                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                    },
                                    RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "author".into(),
                                        descriptor: RSReference::new("Author".into()).into(),
                                    },
                                ]
                                .into(),
                            })]
                            .into(),
                        },
                    },
                ]
            },
        )
    }

    #[test]
    fn test_convert_glob() {
        let config = GTConfig::from_root("module", "./examples/glob");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, &rs_config).unwrap(),
            RSProject {
                modules: vec![
                    RSProjectModule {
                        name: "author".into(),
                        path: "rs/src/author.rs".into(),
                        module: RSModule {
                            doc: None,
                            imports: vec![RSUse {
                                path: "serde".into(),
                                reference: RSUseReference::Named(vec![
                                    RSUseName::Name("Deserialize".into()),
                                    RSUseName::Name("Serialize".into())
                                ]),
                                dependency: RSDependency::Serde,
                            }],
                            definitions: vec![
                                RSDefinition::Struct(RSStruct {
                                    doc: None,
                                    attributes: vec![
                                        "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                                            .into()
                                    ],
                                    name: "Author".into(),
                                    fields: vec![RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "name".into(),
                                        descriptor: RSReference::new("AuthorName".into()).into(),
                                    }]
                                    .into(),
                                }),
                                RSDefinition::Alias(RSAlias {
                                    doc: None,
                                    name: "AuthorName".into(),
                                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                }),
                            ]
                            .into()
                        },
                    },
                    RSProjectModule {
                        name: "book".into(),
                        path: "rs/src/book.rs".into(),
                        module: RSModule {
                            doc: None,
                            imports: vec![
                                RSUse {
                                    path: "self::author".into(),
                                    reference: RSUseReference::Module,
                                    dependency: RSDependency::Local("self::author".into()),
                                },
                                RSUse {
                                    path: "serde".into(),
                                    reference: RSUseReference::Named(vec![RSUseName::Name(
                                        "Deserialize".into()
                                    ), RSUseName::Name(
                                        "Serialize".into()
                                    )]),
                                    dependency: RSDependency::Serde,
                                }
                            ],
                            definitions: vec![RSDefinition::Struct(RSStruct {
                                doc: None,
                                attributes: vec![
                                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                                        .into()
                                ],
                                name: "Book".into(),
                                fields: vec![
                                    RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "title".into(),
                                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                                    },
                                    RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "author".into(),
                                        descriptor: RSReference::new("author.Author".into()).into(),
                                    },
                                    RSField {
                                        doc: None,
                                        attributes: vec![],
                                        name: "author_name".into(),
                                        descriptor: RSReference::new("author.AuthorName".into(),)
                                            .into(),
                                    },
                                ]
                                .into(),
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
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, &rs_config)
                .unwrap()
                .render(&rs_config)
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "rs/Cargo.toml".into(),
                        code: r#"

[dependencies]
serde = { version = "1", features = ["derive"] }
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "rs/src/lib.rs".into(),
                        code: r#"pub mod author;
pub mod book;"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/author.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Author {
    name: String,
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "rs/src/book.rs".into(),
                        code: r#"use self::author::Author;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Book {
    title: String,
    author: Author,
}
"#
                        .into()
                    }
                ]
            }
        )
    }

    #[test]
    fn test_render_nested() {
        let config = GTConfig::from_root("module", "./examples/nested");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, &rs_config)
                .unwrap()
                .render(&rs_config)
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "rs/Cargo.toml".into(),
                        code: r#"

[dependencies]
serde = { version = "1", features = ["derive"] }
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/lib.rs".into(),
                        code: r#"pub mod inventory;
pub mod shop;"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/shop/goods/mod.rs".into(),
                        code: r#"pub mod book;"#.into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/shop/mod.rs".into(),
                        code: r#"pub mod goods;"#.into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/inventory.rs".into(),
                        code: r#"use self::shop::goods::book::Book;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Inventory {
    goods: Vec<Book>,
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "rs/src/shop/goods/book.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Book {
    title: String,
}
"#
                        .into()
                    }
                ]
            }
        )
    }
}
