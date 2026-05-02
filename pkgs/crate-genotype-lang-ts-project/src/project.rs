use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TsProject<'a> {
    pub modules: Vec<TsProjectModule>,
    pub config: GtpPkgConfig<'a, TsConfig>,
}

impl<'a> GtlProject<'a> for TsProject<'a> {
    type Module = TsProjectModule;

    type LangConfig = TsConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.pkg_config_ts();

        let modules = project
            .modules
            .iter()
            .map(|(module_path, module)| {
                TsProjectModule::generate(&project.paths.src, config.target, &module_path, &module)
            })
            .collect::<Vec<_>>();

        Ok(Self { modules, config })
    }

    fn dist(&self) -> Result<GtlProjectDist> {
        let exports = self
            .modules
            .iter()
            .filter_map(|module| match module {
                TsProjectModule::Generated(module) => Some(module),
                TsProjectModule::Error(_) => None,
            })
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

        let project_modules = self
            .modules
            .iter()
            .filter_map(|module| match module {
                TsProjectModule::Generated(module) => Some(module),
                TsProjectModule::Error(_) => None,
            })
            .map(|module| {
                let path = self.config.pkg_src_file_path(&module.path);
                let mut context = TsRenderContext {
                    config: &self.config.target.lang,
                    ..Default::default()
                };
                let source = module
                    .module
                    .render(Default::default(), &mut context)
                    .unwrap();
                GtlProjectFile { path, source }
            })
            .collect::<Vec<_>>();

        let mut modules = vec![barrel];

        if self.config.package_enabled() {
            let gitignore = GtlProjectFile {
                path: self.config.pkg_file_path(&".gitignore".into()),
                source: r#"node_modules"#.into(),
            };
            let package_json = self.generate_manifest(&self.dependencies())?;
            modules.insert(0, package_json);
            modules.insert(0, gitignore);
        }

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

    #[test]
    fn test_convert_base() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

        assert_ron_snapshot!(
          TsProject::generate(&project).unwrap().modules,
          @r#"
        [
          Generated(TsProjectModuleGenerated(
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
            mode: types,
          )),
          Generated(TsProjectModuleGenerated(
            path: "book.ts",
            module: TsModule(
              doc: None,
              imports: [
                TsImport(
                  dependency: Local(TsPath("./author")),
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
                      descriptor: Reference(TsReference(
                        identifier: TsIdentifier("Author"),
                        rel: Regular,
                      )),
                      required: true,
                    ),
                  ],
                )),
              ],
            ),
            mode: types,
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_glob() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/glob".into(), None).unwrap();

        assert_ron_snapshot!(
          TsProject::generate(&project).unwrap().modules,
          @r#"
        [
          Generated(TsProjectModuleGenerated(
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
                      descriptor: Reference(TsReference(
                        identifier: TsIdentifier("AuthorName"),
                        rel: Forward,
                      )),
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
            mode: types,
          )),
          Generated(TsProjectModuleGenerated(
            path: "book.ts",
            module: TsModule(
              doc: None,
              imports: [
                TsImport(
                  dependency: Local(TsPath("./author")),
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
                      descriptor: Reference(TsReference(
                        identifier: TsIdentifier("author.Author"),
                        rel: Regular,
                      )),
                      required: true,
                    ),
                    TsProperty(
                      doc: None,
                      name: TsKey("authorName"),
                      descriptor: Reference(TsReference(
                        identifier: TsIdentifier("author.AuthorName"),
                        rel: Regular,
                      )),
                      required: true,
                    ),
                  ],
                )),
              ],
            ),
            mode: types,
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_render() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

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
              source: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
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
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/dependencies".into(), None)
                .unwrap();
        project.config.ts.common.dependencies = IndexMap::from_iter(vec![(
            "genotype_json_types".into(),
            "@genotype/json".into(),
        )]);

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
              source: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
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
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.version = Some("0.2.0".parse().unwrap());

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source,
            @r#"
        {
          "type": "module",
          "version": "0.2.0",
          "exports": {
            ".": "./src/index.ts"
          }
        }
        "#
        );
    }

    #[test]
    fn test_render_prefers_ts_manifest_version_over_global() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.version = Some("0.2.0".parse().unwrap());
        project
            .config
            .ts
            .common
            .manifest
            .insert("version".into(), "0.3.0".into());

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source,
            @r#"
        {
          "type": "module",
          "version": "0.3.0",
          "exports": {
            ".": "./src/index.ts"
          }
        }
        "#
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.ts.lang.mode = TsMode::Zod;

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();

        let package_file = get_package_file(&dist);
        assert_snapshot!(
            package_file.source,
            @r#"
        {
          "type": "module",
          "exports": {
            ".": "./src/index.ts"
          },
          "dependencies": {
            "zod": "^4"
          }
        }
        "#
        );

        let author_file = get_dist_file(&dist, "src/author.ts");
        assert_snapshot!(
            author_file.source,
            @r#"
        import { z } from "zod";

        export const Author = z.object({
          name: z.string()
        });

        export type Author = z.infer<typeof Author>;
        "#
        );
    }

    #[test]
    fn test_render_prefer_alias() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.ts.lang.prefer = TsPrefer::Alias;

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();

        let author_file = get_dist_file(&dist, "src/author.ts");
        assert_snapshot!(
            author_file.source,
            @r#"
        export type Author = {
          name: string;
        };
        "#
        );

        let book_file = get_dist_file(&dist, "src/book.ts");
        assert_snapshot!(
            book_file.source,
            @r#"
        import { Author } from "./author.js";

        export type Book = {
          title: string;
          author: Author;
        };
        "#
        );
    }

    #[test]
    fn test_render_without_package_global() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.package = false;

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/index.ts",
          "examples/basic/dist/author.ts",
          "examples/basic/dist/book.ts",
        ]
        "#
        );
    }

    #[test]
    fn test_render_without_package_target() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.package = true;
        project.config.ts.common.package = Some(false);

        let dist = TsProject::generate(&project).unwrap().dist().unwrap();

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/index.ts",
          "examples/basic/dist/author.ts",
          "examples/basic/dist/book.ts",
        ]
        "#
        );
    }

    fn get_package_file(dist: &GtlProjectDist) -> &GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().contains("package.json"))
            .unwrap()
    }

    fn get_dist_file<'a>(dist: &'a GtlProjectDist, path_suffix: &str) -> &'a GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().ends_with(path_suffix))
            .unwrap()
    }
}
