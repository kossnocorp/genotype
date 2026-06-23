use crate::prelude::internal::*;

pub struct TsCompiler<'a> {
    project: &'a GtProject,
    config: GtlConfig<'a, TsConfig>,
}

impl<'project> GtlCompiler<'project> for TsCompiler<'project> {
    type ProjectModule = TsProjectModule;

    type Manifest<'config>
        = TsManifest<'project, 'config>
    where
        'project: 'config;

    fn lang(&self) -> GtLang {
        GtLang::Ts
    }

    fn project(&self) -> &GtProject {
        self.project
    }

    fn config(&self) -> &GtlConfig<'project, TsConfig> {
        &self.config
    }

    fn new(project: &'project GtProject) -> Self {
        let lang_config = &project.config.ts;
        let config = GtlConfig::new(&project.config, &project.paths, lang_config);
        TsCompiler { project, config }
    }

    fn generate_extra_files(
        &self,
        project: &GtlProject<'_, '_, TsProjectModule>,
    ) -> Option<GtlGenerations<TsProjectModule>> {
        let (barrel_file, notices) = self.generate_barrel_file(&project.modules);
        Some((vec![barrel_file], Some(notices)))
    }

    fn gitignore_source_code(&self) -> Option<String> {
        Some("node_modules".into())
    }
}

impl TsCompiler<'_> {
    fn generate_barrel_file(
        &self,
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<TsProjectModule>>,
    ) -> (GtlGeneration<TsProjectModule>, Vec<GtNotice>) {
        let mut notices = vec![];
        let mut export_lines = vec![];
        let mut failed_to_render_modules_count = 0;
        let mut failed_to_format_module_path_count = 0;

        for module in modules.values() {
            match module {
                GtlProjectModuleState::Rendered(rendered) => {
                    let path_str = self.format_module_path(&rendered.converted().target_path);
                    match path_str {
                        Ok(path_str) => {
                            export_lines.push(format!(r#"export * from "./{path_str}";"#))
                        }

                        Err(err) => {
                            notices.push(GtNotice::error(format!(
                                "Failed to format module path for barrel file: {err:?}"
                            )));
                            failed_to_format_module_path_count += 1;
                        }
                    }
                }

                _ => {
                    failed_to_render_modules_count += 1;
                }
            }
        }

        let path = self.config.pkg_src_file_path(&"index.ts".into());

        let notice = match (
            failed_to_render_modules_count,
            failed_to_format_module_path_count,
        ) {
            (0, 0) => None,

            (failed_render_count, failed_format_count) => {
                let mut components = vec![];

                if failed_render_count > 0 {
                    components.push(format!(
                        "{} that failed to render",
                        pluralize("module", failed_render_count, true)
                    ));
                }

                if failed_format_count > 0 {
                    components.push(format!(
                        "{} that failed to format",
                        pluralize("export line", failed_format_count, true)
                    ));
                }

                Some(GtNotice::warning(format!(
                    "Barrel file `{path}` rendered, but it excludes {}",
                    components.join(" and ")
                )))
            }
        };

        export_lines.push("".into());

        let source_code = export_lines.join("\n");

        let generation = (
            GtlProjectFileExtraGenerated { path, source_code }.into(),
            notice,
        )
            .into();

        (generation, notices)
    }

    fn format_module_path(&self, target_path: &GtpTargetFilePath) -> Result<String> {
        let path = target_path
            .relative_path()
            .strip_prefix(self.config().pkg_src_path().relative_path())
            .map_err(|err| miette!("Failed to strip prefix: {err:?}"))?;
        Ok(self
            .config()
            .lang_config
            .lang
            .format_module_path(&path.into()))
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
          modules(&project),
          @r#"
        {
          "examples/basic/src/author.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/basic/src/author.type",
              target_path: "examples/basic/dist/ts/src/author.ts",
              project_module: TsProjectModule(
                module: TsModule(
                  doc: None,
                  imports: [],
                  definitions: [
                    Interface(TsInterface(
                      doc: None,
                      name: TsIdentifier("Author"),
                      generics: [],
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
            ),
            resolved_module: TsProjectModule(
              module: TsModule(
                doc: None,
                imports: [],
                definitions: [
                  Interface(TsInterface(
                    doc: None,
                    name: TsIdentifier("Author"),
                    generics: [],
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
          )),
          "examples/basic/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/basic/src/book.type",
              target_path: "examples/basic/dist/ts/src/book.ts",
              project_module: TsProjectModule(
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
                      generics: [],
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
                            arguments: [],
                            rel: Regular,
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: TsProjectModule(
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
                    generics: [],
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
                          arguments: [],
                          rel: Regular,
                        )),
                        required: true,
                      ),
                    ],
                  )),
                ],
              ),
            ),
          )),
        }
        "#
        );
    }

    #[test]
    fn test_convert_glob() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/glob".into(), None).unwrap();

        assert_ron_snapshot!(
          modules(&project),
          @r#"
        {
          "examples/glob/src/author.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/glob/src/author.type",
              target_path: "examples/glob/dist/ts/src/author.ts",
              project_module: TsProjectModule(
                module: TsModule(
                  doc: None,
                  imports: [],
                  definitions: [
                    Interface(TsInterface(
                      doc: None,
                      name: TsIdentifier("Author"),
                      generics: [],
                      extensions: [],
                      properties: [
                        TsProperty(
                          doc: None,
                          name: TsKey("name"),
                          descriptor: Reference(TsReference(
                            identifier: TsIdentifier("AuthorName"),
                            arguments: [],
                            rel: Forward,
                          )),
                          required: true,
                        ),
                      ],
                    )),
                    Alias(TsAlias(
                      doc: None,
                      name: TsIdentifier("AuthorName"),
                      generics: [],
                      descriptor: Primitive(String),
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: TsProjectModule(
              module: TsModule(
                doc: None,
                imports: [],
                definitions: [
                  Interface(TsInterface(
                    doc: None,
                    name: TsIdentifier("Author"),
                    generics: [],
                    extensions: [],
                    properties: [
                      TsProperty(
                        doc: None,
                        name: TsKey("name"),
                        descriptor: Reference(TsReference(
                          identifier: TsIdentifier("AuthorName"),
                          arguments: [],
                          rel: Forward,
                        )),
                        required: true,
                      ),
                    ],
                  )),
                  Alias(TsAlias(
                    doc: None,
                    name: TsIdentifier("AuthorName"),
                    generics: [],
                    descriptor: Primitive(String),
                  )),
                ],
              ),
            ),
          )),
          "examples/glob/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/glob/src/book.type",
              target_path: "examples/glob/dist/ts/src/book.ts",
              project_module: TsProjectModule(
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
                      generics: [],
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
                            arguments: [],
                            rel: Regular,
                          )),
                          required: true,
                        ),
                        TsProperty(
                          doc: None,
                          name: TsKey("authorName"),
                          descriptor: Reference(TsReference(
                            identifier: TsIdentifier("author.AuthorName"),
                            arguments: [],
                            rel: Regular,
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: TsProjectModule(
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
                    generics: [],
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
                          arguments: [],
                          rel: Regular,
                        )),
                        required: true,
                      ),
                      TsProperty(
                        doc: None,
                        name: TsKey("authorName"),
                        descriptor: Reference(TsReference(
                          identifier: TsIdentifier("author.AuthorName"),
                          arguments: [],
                          rel: Regular,
                        )),
                        required: true,
                      ),
                    ],
                  )),
                ],
              ),
            ),
          )),
        }
        "#
        );
    }

    #[test]
    fn test_render() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

        assert_ron_snapshot!(
          compile(&project),
          @r#"
        GtlDist(
          files: [
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/ts/.gitignore",
              source_code: "node_modules",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/ts/package.json",
              source_code: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/ts/src/author.ts",
              source_code: "export interface Author {\n  name: string;\n}\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/ts/src/book.ts",
              source_code: "import { Author } from \"./author.js\";\n\nexport interface Book {\n  title: string;\n  author: Author;\n}\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/ts/src/index.ts",
              source_code: "export * from \"./author.js\";\nexport * from \"./book.js\";\n",
            )),
          ],
          notices: [],
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
          compile(&project),
          @r#"
        GtlDist(
          files: [
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/ts/.gitignore",
              source_code: "node_modules",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/ts/package.json",
              source_code: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/ts/src/index.ts",
              source_code: "export * from \"./prompt.js\";\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/ts/src/prompt.ts",
              source_code: "import { JsonAny } from \"@genotype/json\";\n\nexport interface Prompt {\n  content: string;\n  output: JsonAny;\n}\n",
            )),
          ],
          notices: [],
        )
        "#
        );
    }

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.version = Some("0.2.0".parse().unwrap());

        let dist = compile(&project);
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source_code,
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

        let dist = compile(&project);
        let package_file = get_package_file(&dist);

        assert_snapshot!(
            package_file.source_code,
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
    fn test_render_zod() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.ts.lang.mode = TsMode::Zod;

        let dist = compile(&project);

        let package_file = get_package_file(&dist);
        assert_snapshot!(
            package_file.source_code,
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
            author_file.source_code,
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

        let dist = compile(&project);

        let author_file = get_dist_file(&dist, "src/author.ts");
        assert_snapshot!(
            author_file.source_code,
            @r#"
        export type Author = {
          name: string;
        };
        "#
        );

        let book_file = get_dist_file(&dist, "src/book.ts");
        assert_snapshot!(
            book_file.source_code,
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

        let dist = compile(&project);

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/ts/author.ts",
          "examples/basic/dist/ts/book.ts",
          "examples/basic/dist/ts/index.ts",
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

        let dist = compile(&project);

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/ts/author.ts",
          "examples/basic/dist/ts/book.ts",
          "examples/basic/dist/ts/index.ts",
        ]
        "#
        );
    }

    #[test]
    fn test_dist_includes_module_errors() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        let path: GtpModulePath = "examples/basic/src/broken.type".into();
        let source = GtpModuleSource::Entry { path: path.clone() };
        project.modules.insert(
            path.clone(),
            GtpModule::Error(
                source,
                GtpModuleError::Init {
                    path,
                    message: "synthetic parse failure".into(),
                },
            ),
        );

        let dist = compile(&project);

        assert!(dist.files.iter().any(|file| matches!(
            file,
            GtlDistFile::Error(error)
                if error.path.as_str() == "examples/basic/dist/ts/src/broken.ts"
                    && error.message.contains("Failed to convert")
        )));
    }

    #[test]
    fn test_render_generics() {
        let project = GtpRuntimeSystem::new_and_load_all_modules(
            &"../../examples/04-tests/generics/".into(),
            Some(&"../../examples/04-tests/generics/genotype.ts-interface.toml".into()),
        )
        .unwrap();

        assert_ron_snapshot!(
          compile(&project),
          @r#"
        GtlDist(
          files: [
            Generated(GtlDistFileGenerated(
              path: "../../examples/04-tests/generics/dist/ts-interface/ts/.gitignore",
              source_code: "node_modules",
            )),
            Generated(GtlDistFileGenerated(
              path: "../../examples/04-tests/generics/dist/ts-interface/ts/package.json",
              source_code: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  },\n  \"name\": \"genotype-test-generics-interface-types\",\n  \"version\": \"0.1.0\"\n}",
            )),
            Generated(GtlDistFileGenerated(
              path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/generics.ts",
              source_code: "export type Response<Payload> = ResponseSuccess<Payload> | ResponseFailure;\n\nexport interface ResponseSuccess<Payload> {\n  status: \"success\";\n  value: Payload;\n}\n\nexport interface ResponseFailure {\n  status: \"failure\";\n  error: string;\n}\n\nexport type ResponseString = Response<string>;\n\nexport type ResponsePair = Response<import(\"./pair.js\").Pair<string, number>>;\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/index.ts",
              source_code: "export * from \"./generics.js\";\nexport * from \"./pair.js\";\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/pair.ts",
              source_code: "export interface Pair<Left, Right> {\n  left: Left;\n  right: Right;\n}\n",
            )),
          ],
          notices: [],
        )
        "#
        );
    }

    fn modules(project: &GtProject) -> GtlProjectModules<TsProjectModule> {
        let compiler = TsCompiler::new(project);
        let mut lang_project = GtlProject::<TsProjectModule>::new(compiler.config());
        lang_project.convert(&project.modules);
        lang_project.resolve().unwrap();
        lang_project.modules
    }

    fn compile(project: &GtProject) -> GtlDist {
        TsCompiler::new(project).compile().unwrap().unwrap()
    }

    fn get_package_file(dist: &GtlDist) -> &GtlDistFileGenerated {
        dist.files
            .iter()
            .find_map(|file| match file {
                GtlDistFile::Generated(file) if file.path.as_str().contains("package.json") => {
                    Some(file)
                }
                _ => None,
            })
            .unwrap()
    }

    fn get_dist_file<'a>(dist: &'a GtlDist, path_suffix: &str) -> &'a GtlDistFileGenerated {
        dist.files
            .iter()
            .find_map(|file| match file {
                GtlDistFile::Generated(file) if file.path.as_str().ends_with(path_suffix) => {
                    Some(file)
                }
                _ => None,
            })
            .unwrap()
    }
}
