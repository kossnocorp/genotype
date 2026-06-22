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
    ) -> Result<Option<GtlGenerations<TsProjectModule>>, GtlProjectError> {
        let barrel_file = self.generate_barrel_file(&project.modules);
        Ok(Some((vec![barrel_file], None)))
    }

    fn gitignore_source_code(&self) -> Option<String> {
        Some("node_modules".into())
    }
}

impl TsCompiler<'_> {
    fn generate_barrel_file(
        &self,
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<TsProjectModule>>,
    ) -> GtlGeneration<TsProjectModule> {
        let mut export_lines = vec![];
        let mut failed_to_render_count = 0;

        for module in modules.values() {
            match module {
                GtlProjectModuleState::Rendered(rendered) => {
                    let path_str = self.format_module_path(&rendered.converted().target_path);
                    export_lines.push(format!(r#"export * from "./{path_str}";"#));
                }

                _ => {
                    failed_to_render_count += 1;
                }
            }
        }

        let path = self.config.pkg_src_file_path(&"index.ts".into());

        let notice = match failed_to_render_count {
            0 => None,
            failed_count => Some(GtNotice::warning(format!(
                "Barrel file `{path}` rendered, but it excludes {count} that failed to render",
                count = pluralize("module", failed_count, true)
            ))),
        };

        let source_code = export_lines.join("");

        (
            GtlProjectFileExtraGenerated { path, source_code }.into(),
            notice,
        )
            .into()
    }

    fn format_module_path(&self, target_path: &GtpTargetFilePath) -> String {
        self.config()
            .lang_config
            .lang
            .format_module_path(target_path)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_convert_base() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

//         assert_ron_snapshot!(
//           TsProjectOld::generate_old(&project).unwrap().modules(),
//           @r#"
//         [
//           Generated(TsProjectModuleGenerated(
//             path: "author.ts",
//             module: TsModule(
//               doc: None,
//               imports: [],
//               definitions: [
//                 Interface(TsInterface(
//                   doc: None,
//                   name: TsIdentifier("Author"),
//                   generics: [],
//                   extensions: [],
//                   properties: [
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("name"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                   ],
//                 )),
//               ],
//             ),
//             mode: types,
//           )),
//           Generated(TsProjectModuleGenerated(
//             path: "book.ts",
//             module: TsModule(
//               doc: None,
//               imports: [
//                 TsImport(
//                   dependency: Local(TsPath("./author")),
//                   reference: Named([
//                     Name(TsIdentifier("Author")),
//                   ]),
//                 ),
//               ],
//               definitions: [
//                 Interface(TsInterface(
//                   doc: None,
//                   name: TsIdentifier("Book"),
//                   generics: [],
//                   extensions: [],
//                   properties: [
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("title"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("author"),
//                       descriptor: Reference(TsReference(
//                         identifier: TsIdentifier("Author"),
//                         arguments: [],
//                         rel: Regular,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                 )),
//               ],
//             ),
//             mode: types,
//           )),
//         ]
//         "#
//         );
//     }

//     #[test]
//     fn test_convert_glob() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/glob".into(), None).unwrap();

//         assert_ron_snapshot!(
//           TsProjectOld::generate_old(&project).unwrap().modules(),
//           @r#"
//         [
//           Generated(TsProjectModuleGenerated(
//             path: "author.ts",
//             module: TsModule(
//               doc: None,
//               imports: [],
//               definitions: [
//                 Interface(TsInterface(
//                   doc: None,
//                   name: TsIdentifier("Author"),
//                   generics: [],
//                   extensions: [],
//                   properties: [
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("name"),
//                       descriptor: Reference(TsReference(
//                         identifier: TsIdentifier("AuthorName"),
//                         arguments: [],
//                         rel: Forward,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                 )),
//                 Alias(TsAlias(
//                   doc: None,
//                   name: TsIdentifier("AuthorName"),
//                   generics: [],
//                   descriptor: Primitive(String),
//                 )),
//               ],
//             ),
//             mode: types,
//           )),
//           Generated(TsProjectModuleGenerated(
//             path: "book.ts",
//             module: TsModule(
//               doc: None,
//               imports: [
//                 TsImport(
//                   dependency: Local(TsPath("./author")),
//                   reference: Glob("author"),
//                 ),
//               ],
//               definitions: [
//                 Interface(TsInterface(
//                   doc: None,
//                   name: TsIdentifier("Book"),
//                   generics: [],
//                   extensions: [],
//                   properties: [
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("title"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("author"),
//                       descriptor: Reference(TsReference(
//                         identifier: TsIdentifier("author.Author"),
//                         arguments: [],
//                         rel: Regular,
//                       )),
//                       required: true,
//                     ),
//                     TsProperty(
//                       doc: None,
//                       name: TsKey("authorName"),
//                       descriptor: Reference(TsReference(
//                         identifier: TsIdentifier("author.AuthorName"),
//                         arguments: [],
//                         rel: Regular,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                 )),
//               ],
//             ),
//             mode: types,
//           )),
//         ]
//         "#
//         );
//     }

//     #[test]
//     fn test_render() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

//         assert_ron_snapshot!(
//           TsProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/ts/.gitignore",
//               source: "node_modules",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/ts/package.json",
//               source: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/ts/src/index.ts",
//               source: "export * from \"./author.js\";\nexport * from \"./book.js\";\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/ts/src/author.ts",
//               source: "export interface Author {\n  name: string;\n}\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/ts/src/book.ts",
//               source: "import { Author } from \"./author.js\";\n\nexport interface Book {\n  title: string;\n  author: Author;\n}\n",
//             )),
//           ],
//         )
//         "#
//         );
//     }

//     #[test]
//     fn test_render_dependencies() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/dependencies".into(), None)
//                 .unwrap();
//         project.config.ts.common.dependencies = IndexMap::from_iter(vec![(
//             "genotype_json_types".into(),
//             "@genotype/json".into(),
//         )]);

//         assert_ron_snapshot!(
//           TsProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/ts/.gitignore",
//               source: "node_modules",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/ts/package.json",
//               source: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  }\n}",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/ts/src/index.ts",
//               source: "export * from \"./prompt.js\";\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/ts/src/prompt.ts",
//               source: "import { JsonAny } from \"@genotype/json\";\n\nexport interface Prompt {\n  content: string;\n  output: JsonAny;\n}\n",
//             )),
//           ],
//         )
//         "#
//         );
//     }

//     #[test]
//     fn test_render_uses_global_version_by_default() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.version = Some("0.2.0".parse().unwrap());

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();
//         let package_file = get_package_file(&dist);

//         assert_snapshot!(
//             package_file.source,
//             @r#"
//         {
//           "type": "module",
//           "version": "0.2.0",
//           "exports": {
//             ".": "./src/index.ts"
//           }
//         }
//         "#
//         );
//     }

//     #[test]
//     fn test_render_prefers_ts_manifest_version_over_global() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.version = Some("0.2.0".parse().unwrap());
//         project
//             .config
//             .ts
//             .common
//             .manifest
//             .insert("version".into(), "0.3.0".into());

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();
//         let package_file = get_package_file(&dist);

//         assert_snapshot!(
//             package_file.source,
//             @r#"
//         {
//           "type": "module",
//           "version": "0.3.0",
//           "exports": {
//             ".": "./src/index.ts"
//           }
//         }
//         "#
//         );
//     }

//     #[test]
//     fn test_render_zod() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.ts.lang.mode = TsMode::Zod;

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         let package_file = get_package_file(&dist);
//         assert_snapshot!(
//             package_file.source,
//             @r#"
//         {
//           "type": "module",
//           "exports": {
//             ".": "./src/index.ts"
//           },
//           "dependencies": {
//             "zod": "^4"
//           }
//         }
//         "#
//         );

//         let author_file = get_dist_file(&dist, "src/author.ts");
//         assert_snapshot!(
//             author_file.source,
//             @r#"
//         import { z } from "zod";

//         export const Author = z.object({
//           name: z.string()
//         });

//         export type Author = z.infer<typeof Author>;
//         "#
//         );
//     }

//     #[test]
//     fn test_render_prefer_alias() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.ts.lang.prefer = TsPrefer::Alias;

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         let author_file = get_dist_file(&dist, "src/author.ts");
//         assert_snapshot!(
//             author_file.source,
//             @r#"
//         export type Author = {
//           name: string;
//         };
//         "#
//         );

//         let book_file = get_dist_file(&dist, "src/book.ts");
//         assert_snapshot!(
//             book_file.source,
//             @r#"
//         import { Author } from "./author.js";

//         export type Book = {
//           title: string;
//           author: Author;
//         };
//         "#
//         );
//     }

//     #[test]
//     fn test_render_without_package_global() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.package = false;

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         assert_ron_snapshot!(
//           dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
//           @r#"
//         [
//           "examples/basic/dist/ts/index.ts",
//           "examples/basic/dist/ts/author.ts",
//           "examples/basic/dist/ts/book.ts",
//         ]
//         "#
//         );
//     }

//     #[test]
//     fn test_render_without_package_target() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.package = true;
//         project.config.ts.common.package = Some(false);

//         let dist = TsProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         assert_ron_snapshot!(
//           dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
//           @r#"
//         [
//           "examples/basic/dist/ts/index.ts",
//           "examples/basic/dist/ts/author.ts",
//           "examples/basic/dist/ts/book.ts",
//         ]
//         "#
//         );
//     }

//     // #[test]
//     // fn test_dist_includes_module_errors() {
//     //     let project =
//     //         GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//     //     let mut ts_project = TsProject::generate(&project).unwrap();
//     //     ts_project.modules.push(TsProjectModule::Error(
//     //         TsProjectModuleError::ProjectModuleError {
//     //             path: "examples/basic/src/broken.type".into(),
//     //             target_path: GtpPkgSrcDirRelativePath::from_str("broken.ts"),
//     //             message: "synthetic parse failure".into(),
//     //         },
//     //     ));

//     //     let dist = ts_project.dist().unwrap();

//     //     assert!(dist.files.iter().any(|file| matches!(
//     //         file,
//     //         GtlProjectFile::Error(error)
//     //             if error.path.as_str() == "examples/basic/dist/ts/src/broken.ts"
//     //                 && error.message.contains("synthetic parse failure")
//     //     )));
//     // }

//     #[test]
//     fn test_render_generics() {
//         let project = GtpRuntimeSystem::new_and_load_all_modules(
//             &"../../examples/04-tests/generics/".into(),
//             Some(&"../../examples/04-tests/generics/genotype.ts-interface.toml".into()),
//         )
//         .unwrap();

//         assert_ron_snapshot!(
//           TsProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "../../examples/04-tests/generics/dist/ts-interface/ts/.gitignore",
//               source: "node_modules",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "../../examples/04-tests/generics/dist/ts-interface/ts/package.json",
//               source: "{\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": \"./src/index.ts\"\n  },\n  \"name\": \"genotype-test-generics-interface-types\",\n  \"version\": \"0.1.0\"\n}",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/index.ts",
//               source: "export * from \"./generics.js\";\nexport * from \"./pair.js\";\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/generics.ts",
//               source: "export type Response<Payload> = ResponseSuccess<Payload> | ResponseFailure;\n\nexport interface ResponseSuccess<Payload> {\n  status: \"success\";\n  value: Payload;\n}\n\nexport interface ResponseFailure {\n  status: \"failure\";\n  error: string;\n}\n\nexport type ResponseString = Response<string>;\n\nexport type ResponsePair = Response<import(\"./pair.js\").Pair<string, number>>;\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "../../examples/04-tests/generics/dist/ts-interface/ts/src/pair.ts",
//               source: "export interface Pair<Left, Right> {\n  left: Left;\n  right: Right;\n}\n",
//             )),
//           ],
//         )
//         "#
//         );
//     }

//     fn get_package_file(dist: &GtlProjectDistOld) -> &GtlProjectFileCompiledOld {
//         dist.files
//             .iter()
//             .find_map(|file| match file {
//                 GtlProjectFileOld::Compiled(file) if file.path.as_str().contains("package.json") => {
//                     Some(file)
//                 }
//                 _ => None,
//             })
//             .unwrap()
//     }

//     fn get_dist_file<'a>(
//         dist: &'a GtlProjectDistOld,
//         path_suffix: &str,
//     ) -> &'a GtlProjectFileCompiledOld {
//         dist.files
//             .iter()
//             .find_map(|file| match file {
//                 GtlProjectFileOld::Compiled(file) if file.path.as_str().ends_with(path_suffix) => {
//                     Some(file)
//                 }
//                 _ => None,
//             })
//             .unwrap()
//     }
// }
