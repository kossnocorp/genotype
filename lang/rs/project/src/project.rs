use std::{collections::HashSet, path::PathBuf};

use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
    source::GTLangProjectSource,
};
use genotype_lang_rs_config::RSProjectConfig;
use genotype_lang_rs_tree::{rs_indent, RSRender};
use genotype_project::project::GTProject;
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

        //         let dependencies = self
        //             .modules
        //             .iter()
        //             .flat_map(|module| {
        //                 module
        //                     .module
        //                     .imports
        //                     .iter()
        //                     .map(|import| import.dependency.clone())
        //             })
        //             .collect::<HashSet<_>>();

        //         let cargo = GTLangProjectSource {
        //             path: config.package_path("Cargo.toml".into()),
        //             code: format!(
        //                 r#"[tool.poetry]{}
        // packages = [{{ include = "{}" }}]

        // [tool.poetry.dependencies]
        // {}{}

        // [build-system]
        // requires = ["poetry-core"]
        // build-backend = "poetry.core.masonry.api"
        // "#,
        //                 if let Some(package) = &config.package {
        //                     format!("\n{}", package)
        //                 } else {
        //                     "".into()
        //                 },
        //                 config.module,
        //                 config.lang.version.as_dependency_str(),
        //                 dependencies.iter().fold("".into(), |acc, dependency| {
        //                     if let Some(str) = dependency.external_str() {
        //                         format!("{acc}\n{str}")
        //                     } else {
        //                         acc
        //                     }
        //                 })
        //             ),
        //         };

        //         let mut imports = vec![];
        //         let mut exports = vec![];
        //         for module in self.modules.iter() {
        //             let mut definitions = vec![];
        //             for definition in module.module.definitions.iter() {
        //                 let name = definition.name();
        //                 definitions.push(name.0.clone());
        //                 exports.push(format!("\"{}\"", name.0.clone()));
        //             }

        //             imports.push(format!(
        //                 "from .{} import {}",
        //                 module.name.clone(),
        //                 definitions.join(", ")
        //             ));
        //         }

        //         let init = GTLangProjectSource {
        //             path: config.source_path("__init__.rs".into()),
        //             code: format!(
        //                 "{}\n\n\n__all__ = [{}]",
        //                 imports.join("\n"),
        //                 exports.join(", ")
        //             ),
        //         };

        //         let rs_typed = GTLangProjectSource {
        //             path: config.source_path("rs.typed".into()),
        //             code: "".into(),
        //         };

        //         let module_root = config.module_root_path();
        //         let mut module_paths: HashSet<PathBuf> = HashSet::new();

        //         for module in self.modules.iter() {
        //             // [TODo]
        //             let module_path = module.path.parent().unwrap();
        //             if module_root != module_path {
        //                 module_paths.insert(module_path.into());
        //             }
        //         }

        //         let module_inits = module_paths
        //             .into_iter()
        //             .map(|module_path| GTLangProjectSource {
        //                 path: module_path.join("__init__.rs"),
        //                 code: "".into(),
        //             });

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

        let mut files = vec![gitignore];
        // let mut files = vec![gitignore, cargo, rs_typed, init];
        // files.extend(module_inits);
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
                    //                 GTLangProjectSource {
                    //                     path: "rs/Cargo.toml".into(),
                    //                     code: r#"[tool.poetry]
                    // packages = [{ include = "module" }]

                    // [tool.poetry.dependencies]
                    // rsthon = "^3.12"
                    // genotype-runtime = "^0.4"

                    // [build-system]
                    // requires = ["poetry-core"]
                    // build-backend = "poetry.core.masonry.api"
                    // "#
                    //                     .into(),
                    //                 },
                    GTLangProjectSource {
                        path: "rs/src/author.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

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
                    //                 GTLangProjectSource {
                    //                     path: "rs/rsproject.toml".into(),
                    //                     code: r#"[tool.poetry]
                    // packages = [{ include = "module" }]

                    // [tool.poetry.dependencies]
                    // rsthon = "^3.12"
                    // genotype-runtime = "^0.4"

                    // [build-system]
                    // requires = ["poetry-core"]
                    // build-backend = "poetry.core.masonry.api"
                    // "#
                    //                     .into(),
                    //                 },
                    //                 GTLangProjectSource {
                    //                     path: "rs/src/__init__.rs".into(),
                    //                     code: r#"from .inventory import Inventory
                    // from .shop.goods.book import Book

                    // __all__ = ["Inventory", "Book"]"#
                    //                         .into(),
                    //                 },
                    // GTLangProjectSource {
                    //     path: "rs/src/shop/goods/__init__.rs".into(),
                    //     code: "".into(),
                    // },
                    GTLangProjectSource {
                        path: "rs/src/inventory.rs".into(),
                        code: r#"use self::shop::goods::book::Book;
use serde::{Deserialize, Serialize};

struct Inventory {
    goods: Vec<Book>,
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "rs/src/shop/goods/book.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

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
