use crate::prelude::internal::*;

mod indices;
mod misc;
mod modules;

#[derive(Debug, PartialEq, Clone)]
pub struct RsProject<'a> {
    pub modules: Vec<RsProjectModule>,
    pub config: GtConfigPkg<'a, RsConfig>,
}

impl<'a> GtlProject<'a> for RsProject<'a> {
    type Module = RsProjectModule;

    type LangConfig = RsConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.config.pkg_config_rs();
        let modules = Self::generate_modules(&config.target, &project.modules)?;
        Ok(Self { modules, config })
    }

    fn dist(&self) -> Result<GtlProjectDist> {
        let cargo = self.generate_manifest(&self.dependencies())?;

        let mut files = vec![self.gitignore_source(), cargo];
        files.extend(self.indices_source());
        files.extend(self.modules_source()?);

        Ok(GtlProjectDist { files })
    }

    fn modules(&self) -> Vec<Self::Module> {
        self.modules.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().modules,
            vec![
                RsProjectModule {
                    path: "author.rs".into(),
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
                RsProjectModule {
                    path: "book.rs".into(),
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
                            GtProjectModuleDefinitionResolve {
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
        let config = GtConfig::from_root("module", "./examples/glob");
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().modules,
            vec![
                RsProjectModule {
                    path: "author.rs".into(),
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
                            GtProjectModuleDefinitionResolve {
                                references: HashSet::from_iter(vec![GTReferenceId(
                                    "author".into(),
                                    (19, 29).into()
                                )]),
                                deps: Default::default()
                            }
                        )])
                    },
                },
                RsProjectModule {
                    path: "book.rs".into(),
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
                                    attributes: vec![r#"serde(rename = "authorName")"#.into(),],
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
                                GtProjectModuleDefinitionResolve {
                                    references: HashSet::from_iter(vec![GTReferenceId(
                                        "book".into(),
                                        (72, 82).into()
                                    )]),
                                    deps: Default::default()
                                }
                            ),
                            (
                                GTDefinitionId("author".into(), "Author".into()),
                                GtProjectModuleDefinitionResolve {
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
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/basic/dist/rs/.gitignore".into(),
                        source: r#"target"#.into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/rs/Cargo.toml".into(),
                        source: r#"[package]
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }

"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/rs/src/lib.rs".into(),
                        source: r#"pub(crate) mod author;
pub use author::*;
pub(crate) mod book;
pub use book::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/rs/src/author.rs".into(),
                        source: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
}
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/rs/src/book.rs".into(),
                        source: r#"use super::author::Author;
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
        let config = GtConfig::from_root("module", "./examples/nested");
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/.gitignore".into(),
                        source: r#"target"#.into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/Cargo.toml".into(),
                        source: r#"[package]
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }

"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/src/lib.rs".into(),
                        source: r#"pub(crate) mod inventory;
pub use inventory::*;
pub(crate) mod shop;
pub use shop::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/src/shop/goods/mod.rs".into(),
                        source: r#"pub(crate) mod book;
pub use book::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/src/shop/mod.rs".into(),
                        source: r#"pub(crate) mod goods;
pub use goods::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/src/inventory.rs".into(),
                        source: r#"use super::shop::goods::book::Book;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub goods: Vec<Book>,
}
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/rs/src/shop/goods/book.rs".into(),
                        source: r#"use serde::{Deserialize, Serialize};

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
        let config = GtConfig::from_root("module", "./examples/extensions");
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/.gitignore".into(),
                        source: r#"target"#.into(),
                    },
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/Cargo.toml".into(),
                        source: r#"[package]
edition = "2024"

[dependencies]
litty = "0.2"
serde = { version = "1", features = ["derive"] }

"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/src/lib.rs".into(),
                        source: r#"pub(crate) mod admin;
pub use admin::*;
pub(crate) mod named;
pub use named::*;
pub(crate) mod user;
pub use user::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/src/admin.rs".into(),
                        source: r#"use litty::literal;
use serde::{Deserialize, Serialize};
use crate::named::Name;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Admin {
    pub name: Name,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/src/named.rs".into(),
                        source: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub name: Name,
}

pub type Name = String;
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/extensions/dist/rs/src/user.rs".into(),
                        source: r#"use super::named::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: Name,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
        let config = GtConfig::load(&"./examples/dependencies".into()).unwrap();
        let project = GtProject::load(&config).unwrap();

        assert_eq!(
            RsProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/dependencies/dist/rs/.gitignore".into(),
                        source: r#"target"#.into(),
                    },
                    // [NOTE] The config order is not preserved due to the figment crate missing
                    // the feature for TOML files:
                    // https://github.com/kossnocorp/genotype/issues/36
                    GtlProjectFile {
                        path: "examples/dependencies/dist/rs/Cargo.toml".into(),
                        source: r#"[package]
edition = "2024"
name = "genotype_example_package"
version = "0.1.0"
[dependencies]
genotype_json_types = "0.1.0"
serde = { version = "1", features = ["derive"] }

"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/rs/src/lib.rs".into(),
                        source: r#"pub(crate) mod prompt;
pub use prompt::*;
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/rs/src/prompt.rs".into(),
                        source: r#"use genotype_json_types::JsonAny;
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
