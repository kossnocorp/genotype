use genotype_lang_core_project::project::{GTLangProject, GTLangProjectRender};
use genotype_lang_rs_config::RSProjectConfig;
use genotype_project::project::GTProject;
use miette::Result;

use crate::module::RSProjectModule;

mod cargo;
mod indices;
mod misc;
mod modules;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProject {
    pub modules: Vec<RSProjectModule>,
    pub config: RSProjectConfig,
}

impl GTLangProject<RSProjectConfig> for RSProject {
    fn generate(project: &GTProject, config: RSProjectConfig) -> Result<Self> {
        let modules = Self::generate_modules(project, &config)?;
        Ok(Self { modules, config })
    }

    fn render(&self) -> Result<GTLangProjectRender> {
        let mut files = vec![self.gitignore_source(), self.cargo_source()];
        files.extend(self.indices_source());
        files.extend(self.modules_source()?);

        Ok(GTLangProjectRender { files })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use genotype_config::GTConfig;
    use genotype_lang_core_project::source::GTLangProjectSource;
    use genotype_lang_rs_tree::*;
    use genotype_parser::{GTDefinitionId, GTReferenceId};
    use genotype_project::GTPModuleDefinitionResolve;
    use pretty_assertions::assert_eq;

    use crate::resolve::RSPModuleResolve;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, rs_config).unwrap().modules,
            vec![
                RSProjectModule {
                    name: "author".into(),
                    path: "libs/rs/src/author.rs".into(),
                    module: RSModule {
                        id: "author".into(),
                        doc: None,
                        imports: vec![RSUse {
                            reference: RSUseReference::Named(vec![
                                RSUseName::Name("Deserialize".into()),
                                RSUseName::Name("Serialize".into())
                            ]),
                            dependency: RSDependencyIdent::Serde,
                        }],
                        definitions: vec![RSDefinition::Struct(RSStruct {
                            id: GTDefinitionId("author".into(), "Author".into()),
                            doc: None,
                            attributes: vec![
                                "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
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
                    resolve: RSPModuleResolve {
                        definitions: Default::default()
                    },
                },
                RSProjectModule {
                    name: "book".into(),
                    path: "libs/rs/src/book.rs".into(),
                    module: RSModule {
                        id: "book".into(),
                        doc: None,
                        imports: vec![
                            RSUse {
                                reference: RSUseReference::Named(vec![RSUseName::Name(
                                    "Author".into()
                                )]),
                                dependency: RSDependencyIdent::Local(RSPath(
                                    "author".into(),
                                    "super::author".into()
                                )),
                            },
                            RSUse {
                                reference: RSUseReference::Named(vec![
                                    RSUseName::Name("Deserialize".into()),
                                    RSUseName::Name("Serialize".into())
                                ]),
                                dependency: RSDependencyIdent::Serde,
                            }
                        ],
                        definitions: vec![RSDefinition::Struct(RSStruct {
                            id: GTDefinitionId("book".into(), "Book".into()),
                            doc: None,
                            attributes: vec![
                                "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
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
                                    descriptor: RSReference {
                                        id: GTReferenceId("book".into(), (56, 62).into()),
                                        identifier: "Author".into(),
                                        definition_id: GTDefinitionId(
                                            "author".into(),
                                            "Author".into()
                                        )
                                    }
                                    .into(),
                                },
                            ]
                            .into(),
                        })]
                        .into(),
                    },
                    resolve: RSPModuleResolve {
                        definitions: HashMap::from_iter(vec![(
                            GTDefinitionId("author".into(), "Author".into()),
                            GTPModuleDefinitionResolve {
                                references: HashSet::from_iter(vec![GTReferenceId(
                                    "book".into(),
                                    (56, 62).into()
                                )]),
                                deps: Default::default()
                            }
                        )])
                    },
                },
            ]
        )
    }

    #[test]
    fn test_convert_glob() {
        let config = GTConfig::from_root("module", "./examples/glob");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, rs_config).unwrap().modules,
            vec![
                RSProjectModule {
                    name: "author".into(),
                    path: "libs/rs/src/author.rs".into(),
                    module: RSModule {
                        id: "author".into(),
                        doc: None,
                        imports: vec![RSUse {
                            reference: RSUseReference::Named(vec![
                                RSUseName::Name("Deserialize".into()),
                                RSUseName::Name("Serialize".into())
                            ]),
                            dependency: RSDependencyIdent::Serde,
                        }],
                        definitions: vec![
                            RSDefinition::Struct(RSStruct {
                                id: GTDefinitionId("author".into(), "Author".into()),
                                doc: None,
                                attributes: vec![
                                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)"
                                        .into()
                                ],
                                name: "Author".into(),
                                fields: vec![RSField {
                                    doc: None,
                                    attributes: vec![],
                                    name: "name".into(),
                                    descriptor: RSReference {
                                        id: GTReferenceId("author".into(), (19, 29).into()),
                                        identifier: "AuthorName".into(),
                                        definition_id: GTDefinitionId(
                                            "author".into(),
                                            "AuthorName".into()
                                        )
                                    }
                                    .into(),
                                }]
                                .into(),
                            }),
                            RSDefinition::Alias(RSAlias {
                                id: GTDefinitionId("author".into(), "AuthorName".into()),
                                doc: None,
                                name: "AuthorName".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            }),
                        ]
                        .into()
                    },
                    resolve: RSPModuleResolve {
                        definitions: HashMap::from_iter(vec![(
                            GTDefinitionId("author".into(), "AuthorName".into()),
                            GTPModuleDefinitionResolve {
                                references: HashSet::from_iter(vec![GTReferenceId(
                                    "author".into(),
                                    (19, 29).into()
                                )]),
                                deps: Default::default()
                            }
                        )])
                    },
                },
                RSProjectModule {
                    name: "book".into(),
                    path: "libs/rs/src/book.rs".into(),
                    module: RSModule {
                        id: "book".into(),
                        doc: None,
                        imports: vec![
                            RSUse {
                                reference: RSUseReference::Module,
                                dependency: RSDependencyIdent::Local(RSPath(
                                    "author".into(),
                                    "super::author".into()
                                )),
                            },
                            RSUse {
                                reference: RSUseReference::Named(vec![
                                    RSUseName::Name("Deserialize".into()),
                                    RSUseName::Name("Serialize".into())
                                ]),
                                dependency: RSDependencyIdent::Serde,
                            }
                        ],
                        definitions: vec![RSDefinition::Struct(RSStruct {
                            id: GTDefinitionId("book".into(), "Book".into()),
                            doc: None,
                            attributes: vec![
                                "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()
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
                                    descriptor: RSReference {
                                        id: GTReferenceId("book".into(), (51, 57).into()),
                                        identifier: "author.Author".into(),
                                        definition_id: GTDefinitionId(
                                            "author".into(),
                                            "Author".into()
                                        )
                                    }
                                    .into(),
                                },
                                RSField {
                                    doc: None,
                                    attributes: vec![RSAttribute(
                                        r#"serde(rename = "authorName")"#.into()
                                    )],
                                    name: "author_name".into(),
                                    descriptor: RSReference {
                                        id: GTReferenceId("book".into(), (72, 82).into()),
                                        identifier: "author.AuthorName".into(),
                                        definition_id: GTDefinitionId(
                                            "author".into(),
                                            "AuthorName".into()
                                        )
                                    }
                                    .into(),
                                },
                            ]
                            .into(),
                        })],
                    },
                    resolve: RSPModuleResolve {
                        definitions: HashMap::from_iter(vec![
                            (
                                GTDefinitionId("author".into(), "AuthorName".into()),
                                GTPModuleDefinitionResolve {
                                    references: HashSet::from_iter(vec![GTReferenceId(
                                        "book".into(),
                                        (72, 82).into()
                                    )]),
                                    deps: Default::default()
                                }
                            ),
                            (
                                GTDefinitionId("author".into(), "Author".into()),
                                GTPModuleDefinitionResolve {
                                    references: HashSet::from_iter(vec![GTReferenceId(
                                        "book".into(),
                                        (51, 57).into()
                                    )]),
                                    deps: Default::default()
                                }
                            )
                        ])
                    },
                },
            ]
        )
    }

    #[test]
    fn test_render() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, rs_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/Cargo.toml".into(),
                        code: r#"[package]

[dependencies]
serde = { version = "1", features = ["derive"] }
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/lib.rs".into(),
                        code: r#"mod author;
pub use author::*;
mod book;
pub use book::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/author.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/book.rs".into(),
                        code: r#"use super::author::Author;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: Author,
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
            RSProject::generate(&project, rs_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/Cargo.toml".into(),
                        code: r#"[package]

[dependencies]
serde = { version = "1", features = ["derive"] }
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/lib.rs".into(),
                        code: r#"mod inventory;
pub use inventory::*;
mod shop;
pub use shop::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/shop/goods/mod.rs".into(),
                        code: r#"mod book;
pub use book::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/shop/mod.rs".into(),
                        code: r#"mod goods;
pub use goods::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/inventory.rs".into(),
                        code: r#"use super::shop::goods::book::Book;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub goods: Vec<Book>,
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/shop/goods/book.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
}
"#
                        .into()
                    }
                ]
            }
        )
    }

    #[test]
    fn test_render_extensions() {
        let config = GTConfig::from_root("module", "./examples/extensions");
        let rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            RSProject::generate(&project, rs_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/Cargo.toml".into(),
                        code: r#"[package]

[dependencies]
literals = "0.1"
serde = { version = "1", features = ["derive"] }
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/lib.rs".into(),
                        code: r#"mod admin;
pub use admin::*;
mod named;
pub use named::*;
mod user;
pub use user::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/admin.rs".into(),
                        code: r#"use literals::literal;
use serde::{Deserialize, Serialize};
use crate::named::Name;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Admin {
    pub name: Name,
    pub email: String,
    pub age: Option<i64>,
    pub role: AdminRole,
}

#[literal("superadmin")]
pub struct AdminRoleSuperadmin;

#[literal("admin")]
pub struct AdminRoleAdmin;

#[literal("moderator")]
pub struct AdminRoleModerator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminRole {
    Superadmin(AdminRoleSuperadmin),
    Admin(AdminRoleAdmin),
    Moderator(AdminRoleModerator),
}
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/named.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub name: Name,
}

pub type Name = String;
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/user.rs".into(),
                        code: r#"use super::named::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: Name,
    pub email: String,
    pub age: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
}
"#
                        .into()
                    },
                ]
            }
        )
    }

    #[test]
    fn test_render_dependencies() {
        let config = GTConfig::from_root("module", "./examples/dependencies");
        let mut rs_config = config.as_rust_project();
        let project = GTProject::load(&config).unwrap();

        rs_config.dependencies = Some(HashMap::from_iter(vec![
            ("serde".into(), "1.0".into()),
            ("literals".into(), "0.1".into()),
        ]));

        rs_config.dependencies = Some(HashMap::from_iter(vec![(
            "genotype_json_types".into(),
            "genotype_json".into(),
        )]));

        assert_eq!(
            RSProject::generate(&project, rs_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/Cargo.toml".into(),
                        code: r#"[package]

[dependencies]
serde = { version = "1", features = ["derive"] }
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/lib.rs".into(),
                        code: r#"mod prompt;
pub use prompt::*;
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/rs/src/prompt.rs".into(),
                        code: r#"use genotype_json::JsonAny;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub content: String,
    pub output: JsonAny,
}
"#
                        .into()
                    }
                ]
            }
        )
    }
}
