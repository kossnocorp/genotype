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
    use genotype_test::*;

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
            module: TsModule(
              doc: None,
              imports: [],
              definitions: [
                Interface(TsInterface(
                  doc: None,
                  name: TsIdentifier("Author"),
                  extensions: [],
                  properties: [
                    TsProperty(
                      doc: None,
                      name: TsKey("name"),
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
            module: TsModule(
              doc: None,
              imports: [
                TsImport(
                  path: TsPath("./author"),
                  reference: Named([
                    Name(TsIdentifier("Author")),
                  ]),
                ),
              ],
              definitions: [
                Interface(TsInterface(
                  doc: None,
                  name: TsIdentifier("Book"),
                  extensions: [],
                  properties: [
                    TsProperty(
                      doc: None,
                      name: TsKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    TsProperty(
                      doc: None,
                      name: TsKey("author"),
                      descriptor: Reference(TsReference(TsIdentifier("Author"))),
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
            module: TsModule(
              doc: None,
              imports: [],
              definitions: [
                Interface(TsInterface(
                  doc: None,
                  name: TsIdentifier("Author"),
                  extensions: [],
                  properties: [
                    TsProperty(
                      doc: None,
                      name: TsKey("name"),
                      descriptor: Reference(TsReference(TsIdentifier("AuthorName"))),
                      required: true,
                    ),
                  ],
                )),
                Alias(TsAlias(
                  doc: None,
                  name: TsIdentifier("AuthorName"),
                  descriptor: Primitive(String),
                )),
              ],
            ),
          ),
          TsProjectModule(
            path: "book.ts",
            module: TsModule(
              doc: None,
              imports: [
                TsImport(
                  path: TsPath("./author"),
                  reference: Glob("author"),
                ),
              ],
              definitions: [
                Interface(TsInterface(
                  doc: None,
                  name: TsIdentifier("Book"),
                  extensions: [],
                  properties: [
                    TsProperty(
                      doc: None,
                      name: TsKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    TsProperty(
                      doc: None,
                      name: TsKey("author"),
                      descriptor: Reference(TsReference(TsIdentifier("author.Author"))),
                      required: true,
                    ),
                    TsProperty(
                      doc: None,
                      name: TsKey("authorName"),
                      descriptor: Reference(TsReference(TsIdentifier("author.AuthorName"))),
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

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut config = GtConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());

        let project = GtProject::load(&config).unwrap();

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source,
            @r#"
        {
          "types": "src/index.ts",
          "version": "0.2.0"
        }
        "#
        );
    }

    #[test]
    fn test_render_prefers_ts_manifest_version_over_global() {
        let mut config = GtConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());
        config
            .ts
            .common
            .manifest
            .insert("version".into(), "0.3.0".into());

        let project = GtProject::load(&config).unwrap();

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source,
            @r#"
        {
          "types": "src/index.ts",
          "version": "0.3.0"
        }
        "#
        );
    }

    fn get_package_file<'a>(dist: &'a GtlProjectDist) -> &'a GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().contains("package.json"))
            .unwrap()
    }
}
