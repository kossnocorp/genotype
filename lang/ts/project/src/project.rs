use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
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
                    self.config.target.lang.format_module_path(&module.path)
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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          TsProject::generate(&project).unwrap().modules,
          @r#"
        [
          TsProjectModule(
            path: "author.ts",
            module: TSModule(
              doc: None,
              imports: [],
              definitions: [
                Interface(TSInterface(
                  doc: None,
                  name: TSIdentifier("Author"),
                  extensions: [],
                  properties: [
                    TSProperty(
                      doc: None,
                      name: TSKey("name"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                  ],
                )),
              ],
            ),
          ),
          TsProjectModule(
            path: "book.ts",
            module: TSModule(
              doc: None,
              imports: [
                TSImport(
                  path: TSPath("./author"),
                  reference: Named([
                    Name(TSIdentifier("Author")),
                  ]),
                ),
              ],
              definitions: [
                Interface(TSInterface(
                  doc: None,
                  name: TSIdentifier("Book"),
                  extensions: [],
                  properties: [
                    TSProperty(
                      doc: None,
                      name: TSKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    TSProperty(
                      doc: None,
                      name: TSKey("author"),
                      descriptor: Reference(TSReference(TSIdentifier("Author"))),
                      required: true,
                    ),
                  ],
                )),
              ],
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
          TsProject::generate(&project).unwrap().modules,
          @r#"
        [
          TsProjectModule(
            path: "author.ts",
            module: TSModule(
              doc: None,
              imports: [],
              definitions: [
                Interface(TSInterface(
                  doc: None,
                  name: TSIdentifier("Author"),
                  extensions: [],
                  properties: [
                    TSProperty(
                      doc: None,
                      name: TSKey("name"),
                      descriptor: Reference(TSReference(TSIdentifier("AuthorName"))),
                      required: true,
                    ),
                  ],
                )),
                Alias(TSAlias(
                  doc: None,
                  name: TSIdentifier("AuthorName"),
                  descriptor: Primitive(String),
                )),
              ],
            ),
          ),
          TsProjectModule(
            path: "book.ts",
            module: TSModule(
              doc: None,
              imports: [
                TSImport(
                  path: TSPath("./author"),
                  reference: Glob("author"),
                ),
              ],
              definitions: [
                Interface(TSInterface(
                  doc: None,
                  name: TSIdentifier("Book"),
                  extensions: [],
                  properties: [
                    TSProperty(
                      doc: None,
                      name: TSKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    TSProperty(
                      doc: None,
                      name: TSKey("author"),
                      descriptor: Reference(TSReference(TSIdentifier("author.Author"))),
                      required: true,
                    ),
                    TSProperty(
                      doc: None,
                      name: TSKey("authorName"),
                      descriptor: Reference(TSReference(TSIdentifier("author.AuthorName"))),
                      required: true,
                    ),
                  ],
                )),
              ],
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
          TsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/basic/dist/ts/.gitignore",
              source: "node_modules",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/ts/package.json",
              source: "{\n  \"types\": \"src/index.ts\"\n}",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/ts/src/index.ts",
              source: "export * from \"./author.js\";\nexport * from \"./book.js\";\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/ts/src/author.ts",
              source: "export interface Author {\n  name: string;\n}\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/ts/src/book.ts",
              source: "import { Author } from \"./author.js\";\n\nexport interface Book {\n  title: string;\n  author: Author;\n}\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let mut config = GtConfig::from_root("module", "./examples/dependencies");
        config.ts.common.dependencies = HashMap::from_iter(vec![(
            "genotype_json_types".into(),
            "@genotype/json".into(),
        )]);
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          TsProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/dependencies/dist/ts/.gitignore",
              source: "node_modules",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/ts/package.json",
              source: "{\n  \"types\": \"src/index.ts\"\n}",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/ts/src/index.ts",
              source: "export * from \"./prompt.js\";\n",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/ts/src/prompt.ts",
              source: "import { JsonAny } from \"@genotype/json\";\n\nexport interface Prompt {\n  content: string;\n  output: JsonAny;\n}\n",
            ),
          ],
        )
        "#
        );
    }
}
