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
    use insta::assert_ron_snapshot;

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
            module: RSModule(
              id: GTModuleId("author"),
              doc: None,
              imports: [
                RSUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RSIdentifier("Deserialize")),
                    Name(RSIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RSStruct(
                  id: GTDefinitionId(GTModuleId("author"), "Author"),
                  doc: None,
                  attributes: [
                    RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RSIdentifier("Author"),
                  fields: Resolved([
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("name"),
                      descriptor: Primitive(String),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RSPModuleResolve(
              definitions: {},
            ),
          ),
          RsProjectModule(
            path: "book.rs",
            module: RSModule(
              id: GTModuleId("book"),
              doc: None,
              imports: [
                RSUse(
                  dependency: Local(RSPath(GTModuleId("author"), "super::author")),
                  reference: Named([
                    Name(RSIdentifier("Author")),
                  ]),
                ),
                RSUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RSIdentifier("Deserialize")),
                    Name(RSIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RSStruct(
                  id: GTDefinitionId(GTModuleId("book"), "Book"),
                  doc: None,
                  attributes: [
                    RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RSIdentifier("Book"),
                  fields: Resolved([
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("title"),
                      descriptor: Primitive(String),
                    ),
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("author"),
                      descriptor: Reference(RSReference(
                        id: GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
                        identifier: RSIdentifier("Author"),
                        definition_id: GTDefinitionId(GTModuleId("author"), "Author"),
                      )),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RSPModuleResolve(
              definitions: {
                GTDefinitionId(GTModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                  references: [
                    GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
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
            module: RSModule(
              id: GTModuleId("author"),
              doc: None,
              imports: [
                RSUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RSIdentifier("Deserialize")),
                    Name(RSIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RSStruct(
                  id: GTDefinitionId(GTModuleId("author"), "Author"),
                  doc: None,
                  attributes: [
                    RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RSIdentifier("Author"),
                  fields: Resolved([
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("name"),
                      descriptor: Reference(RSReference(
                        id: GTReferenceId(GTModuleId("author"), GTSpan(18, 28)),
                        identifier: RSIdentifier("AuthorName"),
                        definition_id: GTDefinitionId(GTModuleId("author"), "AuthorName"),
                      )),
                    ),
                  ]),
                )),
                Alias(RSAlias(
                  id: GTDefinitionId(GTModuleId("author"), "AuthorName"),
                  doc: None,
                  name: RSIdentifier("AuthorName"),
                  descriptor: Primitive(String),
                )),
              ],
            ),
            resolve: RSPModuleResolve(
              definitions: {
                GTDefinitionId(GTModuleId("author"), "AuthorName"): GtProjectModuleDefinitionResolve(
                  references: [
                    GTReferenceId(GTModuleId("author"), GTSpan(18, 28)),
                  ],
                  deps: [],
                ),
              },
            ),
          ),
          RsProjectModule(
            path: "book.rs",
            module: RSModule(
              id: GTModuleId("book"),
              doc: None,
              imports: [
                RSUse(
                  dependency: Local(RSPath(GTModuleId("author"), "super::author")),
                  reference: Module,
                ),
                RSUse(
                  dependency: Serde,
                  reference: Named([
                    Name(RSIdentifier("Deserialize")),
                    Name(RSIdentifier("Serialize")),
                  ]),
                ),
              ],
              definitions: [
                Struct(RSStruct(
                  id: GTDefinitionId(GTModuleId("book"), "Book"),
                  doc: None,
                  attributes: [
                    RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                  ],
                  name: RSIdentifier("Book"),
                  fields: Resolved([
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("title"),
                      descriptor: Primitive(String),
                    ),
                    RSField(
                      doc: None,
                      attributes: [],
                      name: RSFieldName("author"),
                      descriptor: Reference(RSReference(
                        id: GTReferenceId(GTModuleId("book"), GTSpan(51, 57)),
                        identifier: RSIdentifier("author.Author"),
                        definition_id: GTDefinitionId(GTModuleId("author"), "Author"),
                      )),
                    ),
                    RSField(
                      doc: None,
                      attributes: [
                        RSAttribute("serde(rename = \"authorName\")"),
                      ],
                      name: RSFieldName("author_name"),
                      descriptor: Reference(RSReference(
                        id: GTReferenceId(GTModuleId("book"), GTSpan(73, 83)),
                        identifier: RSIdentifier("author.AuthorName"),
                        definition_id: GTDefinitionId(GTModuleId("author"), "AuthorName"),
                      )),
                    ),
                  ]),
                )),
              ],
            ),
            resolve: RSPModuleResolve(
              definitions: {
                GTDefinitionId(GTModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                  references: [
                    GTReferenceId(GTModuleId("book"), GTSpan(51, 57)),
                  ],
                  deps: [],
                ),
                GTDefinitionId(GTModuleId("author"), "AuthorName"): GtProjectModuleDefinitionResolve(
                  references: [
                    GTReferenceId(GTModuleId("book"), GTSpan(73, 83)),
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

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/basic/dist/rs/.gitignore",
              source: "target",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/rs/Cargo.toml",
              source: "[package]\nedition = \"2024\"\n\n[dependencies]\nserde = { version = \"1\", features = [\"derive\"] }\n\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/rs/src/lib.rs",
              source: "pub(crate) mod author;\npub use author::*;\npub(crate) mod book;\npub use book::*;\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/rs/src/author.rs",
              source: "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Author {\n    pub name: String,\n}\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/rs/src/book.rs",
              source: "use super::author::Author;\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Book {\n    pub title: String,\n    pub author: Author,\n}\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_nested() {
        let config = GtConfig::from_root("module", "./examples/nested");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/nested/dist/rs/.gitignore",
              source: "target",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/Cargo.toml",
              source: "[package]\nedition = \"2024\"\n\n[dependencies]\nserde = { version = \"1\", features = [\"derive\"] }\n\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/src/lib.rs",
              source: "pub(crate) mod inventory;\npub use inventory::*;\npub(crate) mod shop;\npub use shop::*;\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/src/shop/goods/mod.rs",
              source: "pub(crate) mod book;\npub use book::*;\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/src/shop/mod.rs",
              source: "pub(crate) mod goods;\npub use goods::*;\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/src/inventory.rs",
              source: "use super::shop::goods::book::Book;\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Inventory {\n    pub goods: Vec<Book>,\n}\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/rs/src/shop/goods/book.rs",
              source: "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Book {\n    pub title: String,\n}\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_extensions() {
        let config = GtConfig::from_root("module", "./examples/extensions");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/extensions/dist/rs/.gitignore",
              source: "target",
            ),
            GtlProjectFile(
              path: "examples/extensions/dist/rs/Cargo.toml",
              source: "[package]\nedition = \"2024\"\n\n[dependencies]\nlitty = \"0.2\"\nserde = { version = \"1\", features = [\"derive\"] }\n\n",
            ),
            GtlProjectFile(
              path: "examples/extensions/dist/rs/src/lib.rs",
              source: "pub(crate) mod admin;\npub use admin::*;\npub(crate) mod named;\npub use named::*;\npub(crate) mod user;\npub use user::*;\n",
            ),
            GtlProjectFile(
              path: "examples/extensions/dist/rs/src/admin.rs",
              source: "use litty::literal;\nuse serde::{Deserialize, Serialize};\nuse crate::named::Name;\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Admin {\n    pub name: Name,\n    pub email: String,\n    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub age: Option<i64>,\n    pub role: AdminRole,\n}\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\n#[serde(untagged)]\npub enum AdminRole {\n    #[literal(\"superadmin\")]\n    Superadmin,\n    #[literal(\"admin\")]\n    Admin,\n    #[literal(\"moderator\")]\n    Moderator,\n}\n",
            ),
            GtlProjectFile(
              path: "examples/extensions/dist/rs/src/named.rs",
              source: "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Named {\n    pub name: Name,\n}\n\npub type Name = String;\n",
            ),
            GtlProjectFile(
              path: "examples/extensions/dist/rs/src/user.rs",
              source: "use super::named::Name;\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct User {\n    pub name: Name,\n    pub email: String,\n    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub age: Option<i64>,\n}\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Account {\n    pub email: String,\n}\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let config = GtConfig::load(&"./examples/dependencies".into()).unwrap();
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          RsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/dependencies/dist/rs/.gitignore",
              source: "target",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/rs/Cargo.toml",
              source: "[package]\nedition = \"2024\"\nname = \"genotype_example_package\"\nversion = \"0.1.0\"\n[dependencies]\ngenotype_json_types = \"0.1.0\"\nserde = { version = \"1\", features = [\"derive\"] }\n\n",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/rs/src/lib.rs",
              source: "pub(crate) mod prompt;\npub use prompt::*;\n",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/rs/src/prompt.rs",
              source: "use genotype_json_types::JsonAny;\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Prompt {\n    pub content: String,\n    pub output: JsonAny,\n}\n",
            ),
          ],
        )
        "#
        );
    }
}
