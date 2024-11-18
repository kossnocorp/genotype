use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
};
use genotype_lang_rs_config::RSProjectConfig;
use genotype_project::project::GTProject;
use miette::Result;

use crate::module::RSProjectModule;

mod cargo;
mod indices;
mod modules;
mod misc;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProject {
    pub modules: Vec<RSProjectModule>,
    pub config: RSProjectConfig,
}

impl GTLangProject<RSProjectConfig> for RSProject {
    fn generate(project: &GTProject, config: RSProjectConfig) -> Result<Self> {
        let modules = project
            .modules
            .iter()
            .map(|module| RSProjectModule::generate(&project, module, &config))
            .collect::<Result<_, _>>()?;

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
    use genotype_config::GTConfig;
    use genotype_lang_core_project::source::GTLangProjectSource;
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

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
                        path: "rs/src/author.rs".into(),
                        module: RSModule {
                            id: "author".into(),
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
                            id: "book".into(),
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
                        path: "rs/src/author.rs".into(),
                        module: RSModule {
                            id: "author".into(),
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
                            id: "book".into(),
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
            RSProject::generate(&project, rs_config)
                .unwrap()
                .render()
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

    #[test]
    fn test_render_named() {
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
                        path: "rs/.gitignore".into(),
                        code: r#"target"#.into(),
                    },
                    GTLangProjectSource {
                        path: "rs/Cargo.toml".into(),
                        code: r#"

[dependencies]
serde = { version = "1", features = ["derive"] }
genotype_runtime = "0.1"
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "rs/src/admin.rs".into(),
                        code: r#"use serde::{Deserialize, Serialize};
use genotype_runtime::literal;
                        
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Admin {
    name: String,
    email: String,
    role: AdminRole
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum AdminRole {
    Superadmin,
    Admin,
    Moderator,
}

#[literal("superadmin")]
struct AdminRoleSuperadmin;

#[literal("admin")]
struct AdminRoleAdmin;

#[literal("moderator")]
struct AdminRoleModerator;
"#
                        .into()
                    },
                ]
            }
        )
    }
}
