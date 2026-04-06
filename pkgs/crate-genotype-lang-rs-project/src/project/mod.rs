use crate::prelude::internal::*;

mod indices;
mod misc;
mod modules;

#[derive(Debug, PartialEq, Clone, Serialize)]
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
    use genotype_test::*;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().modules,
          @r#"
        [
          RsProjectModule(
            path: "author.rs",
            module: RsModule(
              id: GtModuleId("author"),
              doc: None,
              imports: [
                RsUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RsIdentifier("Deserialize")),
                    Name(RsIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RsStruct(
                  id: GtDefinitionId(GtModuleId("author"), "Author"),
                  doc: None,
                  attributes: [
                    RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RsIdentifier("Author"),
                  fields: Resolved([
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("name"),
                      descriptor: Primitive(String),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RspModuleResolve(
              definitions: {},
            ),
          ),
          RsProjectModule(
            path: "book.rs",
            module: RsModule(
              id: GtModuleId("book"),
              doc: None,
              imports: [
                RsUse(
                  dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                  reference: Named([
                    Name(RsIdentifier("Author")),
                  ]),
                ),
                RsUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RsIdentifier("Deserialize")),
                    Name(RsIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RsStruct(
                  id: GtDefinitionId(GtModuleId("book"), "Book"),
                  doc: None,
                  attributes: [
                    RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RsIdentifier("Book"),
                  fields: Resolved([
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("title"),
                      descriptor: Primitive(String),
                    ),
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("author"),
                      descriptor: Reference(RsReference(
                        id: GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                        identifier: RsIdentifier("Author"),
                        definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                      )),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RspModuleResolve(
              definitions: {
                GtDefinitionId(GtModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                  references: [
                    GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                  ],
                  deps: [],
                ),
              },
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_glob() {
        let config = GtConfig::from_root("module", "./examples/glob");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().modules,
          @r#"
        [
          RsProjectModule(
            path: "author.rs",
            module: RsModule(
              id: GtModuleId("author"),
              doc: None,
              imports: [
                RsUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RsIdentifier("Deserialize")),
                    Name(RsIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RsStruct(
                  id: GtDefinitionId(GtModuleId("author"), "Author"),
                  doc: None,
                  attributes: [
                    RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RsIdentifier("Author"),
                  fields: Resolved([
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("name"),
                      descriptor: Reference(RsReference(
                        id: GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                        identifier: RsIdentifier("AuthorName"),
                        definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                      )),
                    ),
                  ]),
                )),
                Alias(RsAlias(
                  id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                  doc: None,
                  name: RsIdentifier("AuthorName"),
                  descriptor: Primitive(String),
                )),
              ],
            ),
            resolve: RspModuleResolve(
              definitions: {
                GtDefinitionId(GtModuleId("author"), "AuthorName"): GtProjectModuleDefinitionResolve(
                  references: [
                    GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                  ],
                  deps: [],
                ),
              },
            ),
          ),
          RsProjectModule(
            path: "book.rs",
            module: RsModule(
              id: GtModuleId("book"),
              doc: None,
              imports: [
                RsUse(
                  dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                  reference: Module,
                ),
                RsUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RsIdentifier("Deserialize")),
                    Name(RsIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RsStruct(
                  id: GtDefinitionId(GtModuleId("book"), "Book"),
                  doc: None,
                  attributes: [
                    RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RsIdentifier("Book"),
                  fields: Resolved([
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("title"),
                      descriptor: Primitive(String),
                    ),
                    RsField(
                      doc: None,
                      attributes: [],
                      name: RsFieldName("author"),
                      descriptor: Reference(RsReference(
                        id: GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                        identifier: RsIdentifier("author.Author"),
                        definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                      )),
                    ),
                    RsField(
                      doc: None,
                      attributes: [
                        RsAttribute("serde(rename = \"authorName\")"),
                      ],
                      name: RsFieldName("author_name"),
                      descriptor: Reference(RsReference(
                        id: GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                        identifier: RsIdentifier("author.AuthorName"),
                        definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                      )),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RspModuleResolve(
              definitions: {
                GtDefinitionId(GtModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                  references: [
                    GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                  ],
                  deps: [],
                ),
                GtDefinitionId(GtModuleId("author"), "AuthorName"): GtProjectModuleDefinitionResolve(
                  references: [
                    GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                  ],
                  deps: [],
                ),
              },
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_render() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();

        assert_equal!(dist.files.iter().count(), 5);

        assert_debug_snapshot!(
          dist.files[0].path,
          @r#"
        GtCwdRelativePath(
            "examples/basic/dist/rs/.gitignore",
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source,
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path,
          @r#"
        GtCwdRelativePath(
            "examples/basic/dist/rs/Cargo.toml",
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source,
          @r#"
        [package]
        edition = "2024"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path,
          @r#"
        GtCwdRelativePath(
            "examples/basic/dist/rs/src/lib.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source,
          @"
        pub(crate) mod author;
        pub use author::*;
        pub(crate) mod book;
        pub use book::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[3].path,
          @r#"
        GtCwdRelativePath(
            "examples/basic/dist/rs/src/author.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source,
          @"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Author {
            pub name: String,
        }
        "
        );

        assert_ron_snapshot!(
          dist.files[4].path,
          @r#""examples/basic/dist/rs/src/book.rs""#
        );
        assert_snapshot!(
          dist.files[4].source,
          @"
        use super::author::Author;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Book {
            pub title: String,
            pub author: Author,
        }
        "
        );
    }

    #[test]
    fn test_render_nested() {
        let config = GtConfig::from_root("module", "./examples/nested");
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();

        assert_equal!(dist.files.iter().count(), 7);

        assert_debug_snapshot!(
          dist.files[0].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/.gitignore",
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source,
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/Cargo.toml",
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source,
          @r#"
        [package]
        edition = "2024"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/src/lib.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source,
          @"
        pub(crate) mod inventory;
        pub use inventory::*;
        pub(crate) mod shop;
        pub use shop::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[3].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/src/shop/goods/mod.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source,
          @"
        pub(crate) mod book;
        pub use book::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[4].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/src/shop/mod.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[4].source,
          @"
        pub(crate) mod goods;
        pub use goods::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[5].path,
          @r#"
        GtCwdRelativePath(
            "examples/nested/dist/rs/src/inventory.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[5].source,
          @"
        use super::shop::goods::book::Book;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Inventory {
            pub goods: Vec<Book>,
        }
        "
        );

        assert_ron_snapshot!(
          dist.files[6].path,
          @r#""examples/nested/dist/rs/src/shop/goods/book.rs""#
        );
        assert_snapshot!(
          dist.files[6].source,
          @"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Book {
            pub title: String,
        }
        "
        );
    }

    #[test]
    fn test_render_recursive_box() {
        let config = GtConfig::load(&"./examples/recursive".into()).unwrap();
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();
        let node_file = dist
            .files
            .iter()
            .find(|file| file.path.as_str().contains("src/node.rs"))
            .unwrap();

        assert_snapshot!(
            node_file.source,
            @r#"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Node {
            pub value: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub next: Option<Box<Node>>,
        }
        "#
        );
    }

    #[test]
    fn test_render_recursive_box_with_extensions() {
        let config = GtConfig::load(&"./examples/recursive".into()).unwrap();
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();
        let tree_file = dist
            .files
            .iter()
            .find(|file| file.path.as_str().contains("src/tree.rs"))
            .unwrap();

        assert_snapshot!(
            tree_file.source,
            @r#"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct NodeMeta {
            pub id: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreeLinkFields {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub parent: Option<TreeNode>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreeNode {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub parent: Option<Box<TreeNode>>,
            pub payload: Box<TreePayload>,
            pub children: Vec<TreeNode>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreePayload {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub owner: Option<Box<TreeNode>>,
            pub kind: String,
        }
        "#
        );
    }

    #[test]
    fn test_render_extensions() {
        let config = GtConfig::from_root("module", "./examples/extensions");
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();

        assert_equal!(dist.files.iter().count(), 6);

        assert_debug_snapshot!(
          dist.files[0].path,
          @r#"
        GtCwdRelativePath(
            "examples/extensions/dist/rs/.gitignore",
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source,
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path,
          @r#"
        GtCwdRelativePath(
            "examples/extensions/dist/rs/Cargo.toml",
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source,
          @r#"
        [package]
        edition = "2024"

        [dependencies]
        litty = "0.2"
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path,
          @r#"
        GtCwdRelativePath(
            "examples/extensions/dist/rs/src/lib.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source,
          @"
        pub(crate) mod admin;
        pub use admin::*;
        pub(crate) mod named;
        pub use named::*;
        pub(crate) mod user;
        pub use user::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[3].path,
          @r#"
        GtCwdRelativePath(
            "examples/extensions/dist/rs/src/admin.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source,
          @r#"
        use litty::literal;
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

        #[derive(Debug, Clone, PartialEq, Literals)]
        #[serde(untagged)]
        pub enum AdminRole {
            #[literal("superadmin")]
            Superadmin,
            #[literal("admin")]
            Admin,
            #[literal("moderator")]
            Moderator,
        }
        "#
        );

        assert_debug_snapshot!(
          dist.files[4].path,
          @r#"
        GtCwdRelativePath(
            "examples/extensions/dist/rs/src/named.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[4].source,
          @"
        use litty::Literals;

        #[derive(Debug, Clone, PartialEq, Literals)]
        #[literals(named = true)]
        pub struct Named {
            pub name: Name,
        }

        pub type Name = String;
        "
        );

        assert_ron_snapshot!(
          dist.files[5].path,
          @r#""examples/extensions/dist/rs/src/user.rs""#
        );
        assert_snapshot!(
          dist.files[5].source,
          @r#"
        use super::named::Name;
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
        );
    }

    #[test]
    fn test_render_dependencies() {
        let config = GtConfig::load(&"./examples/dependencies".into()).unwrap();
        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();

        assert_equal!(dist.files.iter().count(), 4);

        assert_debug_snapshot!(
          dist.files[0].path,
          @r#"
        GtCwdRelativePath(
            "examples/dependencies/dist/rs/.gitignore",
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source,
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path,
          @r#"
        GtCwdRelativePath(
            "examples/dependencies/dist/rs/Cargo.toml",
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source,
          @r#"
        [package]
        edition = "2024"
        name = "genotype_example_package"
        version = "0.1.0"
        [dependencies]
        genotype_json_types = "0.1.0"
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path,
          @r#"
        GtCwdRelativePath(
            "examples/dependencies/dist/rs/src/lib.rs",
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source,
          @"
        pub(crate) mod prompt;
        pub use prompt::*;
        "
        );

        assert_ron_snapshot!(
          dist.files[3].path,
          @r#""examples/dependencies/dist/rs/src/prompt.rs""#
        );
        assert_snapshot!(
          dist.files[3].source,
          @"
        use genotype_json_types::JsonAny;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Prompt {
            pub content: String,
            pub output: JsonAny,
        }
        "
        );
    }

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut config = GtConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());

        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();
        let cargo = get_cargo_file(&dist);

        assert_snapshot!(
            cargo.source,
            @r#"
        [package]
        edition = "2024"
        version = "0.2.0"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );
    }

    #[test]
    fn test_render_prefers_rs_manifest_version_over_global() {
        let mut config = GtConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());
        config.rs.common.manifest = toml::from_str(
            r#"[package]
version = "0.3.0"
"#,
        )
        .unwrap();

        let project = GtProject::load(&config).unwrap();

        let dist = RsProject::generate(&project).unwrap().dist().unwrap();
        let cargo = get_cargo_file(&dist);

        assert_snapshot!(
            cargo.source,
            @r#"
        [package]
        edition = "2024"
        version = "0.3.0"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );
    }

    fn get_cargo_file<'a>(dist: &'a GtlProjectDist) -> &'a GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().contains("Cargo.toml"))
            .unwrap()
    }
}
