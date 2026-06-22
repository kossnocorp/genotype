use crate::prelude::internal::*;

pub struct PyCompiler<'project> {
    project: &'project GtProject,
    config: GtlConfig<'project, PyConfig>,
}

impl<'project> GtlCompiler<'project> for PyCompiler<'project> {
    type ProjectModule = PyProjectModule;

    type Manifest<'config>
        = PyManifest<'project, 'config>
    where
        'project: 'config;

    fn lang(&self) -> GtLang {
        GtLang::Py
    }

    fn project(&self) -> &GtProject {
        self.project
    }

    fn config(&self) -> &GtlConfig<'project, PyConfig> {
        &self.config
    }

    fn new(project: &'project GtProject) -> Self {
        let lang_config = &project.config.py;
        let config = GtlConfig::new(&project.config, &project.paths, lang_config);
        PyCompiler { project, config }
    }

    fn generate_extra_files(
        &self,
        project: &GtlProject<'project, '_, PyProjectModule>,
    ) -> Result<Option<GtlGenerations<PyProjectModule>>, GtlProjectError> {
        let mut files = vec![];
        let mut notices = None;

        files.push(self.generate_root_init_file(&project.modules));

        let (module_init_files, notice) = self.generate_module_init_files(&project.modules);
        files.extend(module_init_files);
        if let Some(notice) = notice {
            notices = Some(vec![notice]);
        }

        files.push(self.generate_py_typed_file());

        Ok(Some((files, notices)))
    }

    fn gitignore_source_code(&self) -> Option<String> {
        Some(
            indoc! {"
                __pycache__
                dist
            "}
            .into(),
        )
    }
}

impl PyCompiler<'_> {
    fn generate_root_init_file(
        &self,
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<PyProjectModule>>,
    ) -> GtlGeneration<PyProjectModule> {
        let mut notices = vec![];

        let mut import_lines = vec![];
        let mut exports = vec![];
        let mut failed_count = 0;

        for module in modules.values() {
            match module {
                GtlProjectModuleState::Rendered(rendered) => {
                    let project_module = rendered.project_module();

                    let mut definitions = vec![];
                    for definition in project_module.module().definitions.iter() {
                        let name = definition.name();
                        definitions.push(name.0.clone());
                        exports.push(format!("\"{}\"", name.0.clone()));
                    }

                    let source_path = rendered.source_path();

                    match self.py_module_path(source_path) {
                        Ok(py_module_path) => {
                            let imports = definitions.join(", ");
                            import_lines.push(format!("from .{py_module_path} import {imports}"));
                        }

                        Err(err) => {
                            failed_count += 1;
                            notices.push(GtNotice::error(format!(
                                "Failed to resolve module path for `{source_path}`: {}",
                                err
                            )));
                        }
                    }
                }

                _ => {
                    failed_count += 1;
                }
            }
        }

        let path = self.config.pkg_src_file_path(&"__init__.py".into());

        if failed_count > 0 {
            notices.push(GtNotice::warning(format!(
                "Init file `{path}` rendered, but it excludes {count} that failed to render",
                count = pluralize("module", failed_count, true)
            )));
        }

        let source_code = format!(
            "{}\n\n\n__all__ = [{}]",
            import_lines.join("\n"),
            exports.join(", ")
        );

        (
            GtlProjectFileExtraGenerated { path, source_code }.into(),
            notices,
        )
            .into()
    }

    fn generate_py_typed_file(&self) -> GtlGeneration<PyProjectModule> {
        let path = self.config.pkg_src_file_path(&"py.typed".into());
        let source_code = "".into();
        GtlProjectFileExtraGenerated { path, source_code }.into()
    }

    fn generate_module_init_files(
        &self,
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<PyProjectModule>>,
    ) -> (Vec<GtlGeneration<PyProjectModule>>, Option<GtNotice>) {
        let mut files = vec![];

        let mut module_paths: IndexSet<GtpPkgSrcDirRelativePath> = IndexSet::new();
        let pkg_src_path = self.config().pkg_src_path();

        let mut formatted_errors = vec![];

        for module in modules.values() {
            if let Some(module_target_path) = module.target_path() {
                match module_target_path.relative_path_to(&pkg_src_path) {
                    Ok(rel_path) => {
                        let pkg_src_rel_path = GtpPkgSrcDirRelativePath::new(rel_path);
                        if let Some(module_parent_path) = pkg_src_rel_path.to_parent() {
                            if module_parent_path == ".".into() {
                                continue;
                            }
                            module_paths.insert(module_parent_path.into());
                        }
                    }

                    Err(err) => formatted_errors.push(GtNotice::format_report(err)),
                }
            }
        }

        let notice = if formatted_errors.is_empty() {
            None
        } else {
            Some(GtNotice::warning((
                format!("Some of `__init__.py` could be missing"),
                formatted_errors,
            )))
        };

        for module_path in module_paths {
            let path = self
                .config()
                .pkg_src_file_path(&module_path.join_relative_path(&"__init__.py".into()));
            files.push(GtlGeneration::file(GtlProjectFileExtra::Generated(
                GtlProjectFileExtraGenerated {
                    path,
                    source_code: "".into(),
                },
            )));
        }

        (files, notice)
    }

    fn py_module_path(&self, module_path: &GtpModulePath) -> Result<String> {
        let module_id = module_path.to_module_id(&self.config.project_paths.src)?;
        Ok(PyPath::to_py_module_path(module_id.as_str_without_ext()))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use genotype_test::*;

//     #[test]
//     fn test_convert_base() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

//         assert_ron_snapshot!(
//           PyProjectOld::generate_old(&project).unwrap().modules(),
//           @r#"
//         [
//           Generated(PyProjectModuleGenerated(
//             name: "author",
//             path: "author.py",
//             module: PyModule(
//               doc: None,
//               imports: [
//                 PyImport(
//                   dependency: Runtime,
//                   reference: Named([
//                     Name(PyIdentifier("Model")),
//                   ]),
//                 ),
//               ],
//               definitions: [
//                 Class(PyClass(
//                   doc: None,
//                   name: PyIdentifier("Author"),
//                   extensions: [],
//                   properties: [
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("name"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                   ],
//                   references: [],
//                 )),
//               ],
//             ),
//           )),
//           Generated(PyProjectModuleGenerated(
//             name: "book",
//             path: "book.py",
//             module: PyModule(
//               doc: None,
//               imports: [
//                 PyImport(
//                   dependency: Path(PyPath(".author")),
//                   reference: Named([
//                     Name(PyIdentifier("Author")),
//                   ]),
//                 ),
//                 PyImport(
//                   dependency: Runtime,
//                   reference: Named([
//                     Name(PyIdentifier("Model")),
//                   ]),
//                 ),
//               ],
//               definitions: [
//                 Class(PyClass(
//                   doc: None,
//                   name: PyIdentifier("Book"),
//                   extensions: [],
//                   properties: [
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("title"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("author"),
//                       descriptor: Reference(PyReference(
//                         identifier: PyIdentifier("Author"),
//                         forward: false,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                   references: [
//                     PyIdentifier("Author"),
//                   ],
//                 )),
//               ],
//             ),
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
//           PyProjectOld::generate_old(&project).unwrap().modules(),
//           @r#"
//         [
//           Generated(PyProjectModuleGenerated(
//             name: "author",
//             path: "author.py",
//             module: PyModule(
//               doc: None,
//               imports: [
//                 PyImport(
//                   dependency: Runtime,
//                   reference: Named([
//                     Name(PyIdentifier("Model")),
//                   ]),
//                 ),
//               ],
//               definitions: [
//                 Alias(PyAlias(
//                   doc: None,
//                   name: PyIdentifier("AuthorName"),
//                   descriptor: Primitive(String),
//                   references: [],
//                 )),
//                 Class(PyClass(
//                   doc: None,
//                   name: PyIdentifier("Author"),
//                   extensions: [],
//                   properties: [
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("name"),
//                       descriptor: Reference(PyReference(
//                         identifier: PyIdentifier("AuthorName"),
//                         forward: false,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                   references: [
//                     PyIdentifier("AuthorName"),
//                   ],
//                 )),
//               ],
//             ),
//           )),
//           Generated(PyProjectModuleGenerated(
//             name: "book",
//             path: "book.py",
//             module: PyModule(
//               doc: None,
//               imports: [
//                 PyImport(
//                   dependency: Path(PyPath(".author")),
//                   reference: Default(Some(PyIdentifier("author"))),
//                 ),
//                 PyImport(
//                   dependency: Runtime,
//                   reference: Named([
//                     Name(PyIdentifier("Model")),
//                   ]),
//                 ),
//               ],
//               definitions: [
//                 Class(PyClass(
//                   doc: None,
//                   name: PyIdentifier("Book"),
//                   extensions: [],
//                   properties: [
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("title"),
//                       descriptor: Primitive(String),
//                       required: true,
//                     ),
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("author"),
//                       descriptor: Reference(PyReference(
//                         identifier: PyIdentifier("author.Author"),
//                         forward: false,
//                       )),
//                       required: true,
//                     ),
//                     PyProperty(
//                       doc: None,
//                       name: PyKey("author_name"),
//                       descriptor: Reference(PyReference(
//                         identifier: PyIdentifier("author.AuthorName"),
//                         forward: false,
//                       )),
//                       required: true,
//                     ),
//                   ],
//                   references: [
//                     PyIdentifier("author.Author"),
//                     PyIdentifier("author.AuthorName"),
//                   ],
//                 )),
//               ],
//             ),
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
//           PyProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/.gitignore",
//               source: "__pycache__\ndist",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/pyproject.toml",
//               source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/module/py.typed",
//               source: "",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/module/__init__.py",
//               source: "from .author import Author\nfrom .book import Book\n\n\n__all__ = [\"Author\", \"Book\"]",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/module/author.py",
//               source: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Author(Model):\n    name: str\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/basic/dist/py/module/book.py",
//               source: "from __future__ import annotations\n\n\nfrom .author import Author\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n    author: Author\n",
//             )),
//           ],
//         )
//         "#
//         );
//     }

//     #[test]
//     fn test_render_nested() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/nested".into(), None).unwrap();

//         assert_ron_snapshot!(
//           PyProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/.gitignore",
//               source: "__pycache__\ndist",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/pyproject.toml",
//               source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/module/py.typed",
//               source: "",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/module/__init__.py",
//               source: "from .inventory import Inventory\nfrom .shop.goods.book import Book\n\n\n__all__ = [\"Inventory\", \"Book\"]",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/module/shop/goods/__init__.py",
//               source: "",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/module/inventory.py",
//               source: "from __future__ import annotations\n\n\nfrom .shop.goods.book import Book\nfrom genotype import Model\n\n\nclass Inventory(Model):\n    goods: list[Book]\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/nested/dist/py/module/shop/goods/book.py",
//               source: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n",
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
//         project.config.py.common.dependencies =
//             IndexMap::from_iter(vec![("genotype_json_types".into(), "genotype_json".into())]);

//         assert_ron_snapshot!(
//           PyProjectOld::generate_old(&project).unwrap().dist().unwrap(),
//           @r#"
//         GtlProjectDist(
//           files: [
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/py/.gitignore",
//               source: "__pycache__\ndist",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/py/pyproject.toml",
//               source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/py/module/py.typed",
//               source: "",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/py/module/__init__.py",
//               source: "from .prompt import Prompt\n\n\n__all__ = [\"Prompt\"]",
//             )),
//             Generated(GtlProjectFileGenerated(
//               path: "examples/dependencies/dist/py/module/prompt.py",
//               source: "from __future__ import annotations\n\n\nfrom genotype_json import JsonAny\nfrom genotype import Model\n\n\nclass Prompt(Model):\n    content: str\n    output: JsonAny\n",
//             )),
//           ],
//         )
//         "#
//         );
//     }

//     #[test]
//     fn test_render_cyclic_lists() {
//         let project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/cyclic-lists".into(), None)
//                 .unwrap();

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         let json = get_dist_file(&dist, "module/json.py");
//         assert_snapshot!(
//             json.source,
//             @r#"
//         from __future__ import annotations

//         from typing import Optional, Literal
//         from genotype import Model

//         class JsonBase(Model):
//             name: Optional[str] = None
//             doc: Optional[str] = None

//         class JsonNull(JsonBase, Model):
//             kind: Literal["null"]

//         class JsonBoolean(JsonBase, Model):
//             kind: Literal["boolean"]

//         class JsonNumber(JsonBase, Model):
//             kind: Literal["number"]

//         class JsonString(JsonBase, Model):
//             kind: Literal["string"]

//         class JsonLiteral(JsonBase, Model):
//             kind: Literal["literal"]
//             value: str | float | bool | Literal[None]

//         type JsonLiteralKind = Literal["string"] | Literal["number"] | Literal["boolean"] | Literal["null"]

//         class JsonTuple(JsonBase, Model):
//             kind: Literal["tuple"]
//             descriptors: list[JsonAny]

//         class JsonUnion(JsonBase, Model):
//             kind: Literal["union"]
//             descriptors: list[JsonAny]

//         class JsonProperty(Model):
//             kind: Literal["property"]
//             name: str
//             doc: Optional[str] = None
//             descriptor: JsonAny
//             required: Optional[bool] = None

//         class JsonObject(JsonBase, Model):
//             kind: Literal["object"]
//             properties: list[JsonProperty]

//         class JsonArray(JsonBase, Model):
//             kind: Literal["array"]
//             descriptor: JsonAny

//         type JsonAny = JsonNull | JsonBoolean | JsonNumber | JsonString | JsonArray | JsonObject | JsonUnion | JsonLiteral | JsonTuple
//         "#
//         );

//         let init = get_dist_file(&dist, "module/__init__.py");
//         assert_snapshot!(
//             init.source,
//             @r#"
//         from .json import JsonBase, JsonNull, JsonBoolean, JsonNumber, JsonString, JsonLiteral, JsonLiteralKind, JsonTuple, JsonUnion, JsonProperty, JsonObject, JsonArray, JsonAny

//         __all__ = ["JsonBase", "JsonNull", "JsonBoolean", "JsonNumber", "JsonString", "JsonLiteral", "JsonLiteralKind", "JsonTuple", "JsonUnion", "JsonProperty", "JsonObject", "JsonArray", "JsonAny"]
//         "#
//         );
//     }

//     #[test]
//     fn test_render_uses_global_version_by_default() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.version = Some("0.2.0".parse().unwrap());

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();
//         let pyproject = get_project_file(&dist);

//         assert_snapshot!(
//             pyproject.source,
//             @r#"
//         [tool.poetry]
//         packages = [{ include = "module" }]
//         version = "0.2.0"

//         [tool.poetry.dependencies]
//         python = "^3.13"
//         genotype-runtime = "^0.4"

//         [build-system]
//         requires = ["poetry-core"]
//         build-backend = "poetry.core.masonry.api"
//         "#
//         );
//     }

//     #[test]
//     fn test_render_prefers_py_manifest_version_over_global() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.version = Some("0.2.0".parse().unwrap());
//         project.config.py.common.manifest = toml::from_str(
//             r#"[tool.poetry]
// version = "0.3.0"
// "#,
//         )
//         .unwrap();

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();
//         let pyproject = get_project_file(&dist);

//         assert_snapshot!(
//             pyproject.source,
//             @r#"
//         [tool.poetry]
//         packages = [{ include = "module" }]
//         version = "0.3.0"

//         [tool.poetry.dependencies]
//         python = "^3.13"
//         genotype-runtime = "^0.4"

//         [build-system]
//         requires = ["poetry-core"]
//         build-backend = "poetry.core.masonry.api"
//         "#
//         );
//     }

//     #[test]
//     fn test_render_uv_manifest() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.py.lang.manager = genotype_lang_py_config::PyPackageManager::Uv;
//         project.config.version = Some("0.2.0".parse().unwrap());
//         project.config.py.common.manifest = toml::from_str(
//             r#"[project]
// name = "module"
// "#,
//         )
//         .unwrap();

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();
//         let pyproject = get_project_file(&dist);

//         assert_snapshot!(
//             pyproject.source,
//             @r#"
//         [project]
//         requires-python = ">=3.13,<4"
//         version = "0.2.0"
//         name = "module"
//         dependencies = ["genotype-runtime>=0.4,<0.5"]

//         [build-system]
//         requires = ["hatchling"]
//         build-backend = "hatchling.build"

//         [tool.hatch.build.targets.wheel]
//         packages = ["module"]
//         "#
//         );
//     }

//     #[test]
//     fn test_render_without_package_global() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.package = false;

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         assert_ron_snapshot!(
//           dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
//           @r#"
//         [
//           "examples/basic/dist/py/py.typed",
//           "examples/basic/dist/py/__init__.py",
//           "examples/basic/dist/py/author.py",
//           "examples/basic/dist/py/book.py",
//         ]
//         "#
//         );
//     }

//     #[test]
//     fn test_render_without_package_target() {
//         let mut project =
//             GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//         project.config.package = true;
//         project.config.py.common.package = Some(false);

//         let dist = PyProjectOld::generate_old(&project)
//             .unwrap()
//             .dist()
//             .unwrap();

//         assert_ron_snapshot!(
//           dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
//           @r#"
//         [
//           "examples/basic/dist/py/py.typed",
//           "examples/basic/dist/py/__init__.py",
//           "examples/basic/dist/py/author.py",
//           "examples/basic/dist/py/book.py",
//         ]
//         "#
//         );
//     }

//     // #[test]
//     // fn test_dist_includes_module_errors() {
//     //     let project =
//     //         GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
//     //     let mut py_project = PyProject::generate(&project).unwrap();
//     //     py_project.modules.push(PyProjectModule::Error(
//     //         PyProjectModuleError::ProjectModuleError {
//     //             path: "examples/basic/src/broken.type".into(),
//     //             target_path: GtpPkgSrcDirRelativePath::from_str("broken.py"),
//     //             message: "synthetic parse failure".into(),
//     //         },
//     //     ));

//     //     let dist = py_project.dist().unwrap();

//     //     assert!(dist.files.iter().any(|file| matches!(
//     //         file,
//     //         GtlProjectFile::Error(error)
//     //             if error.path.as_str() == "examples/basic/dist/py/module/broken.py"
//     //                 && error.message.contains("synthetic parse failure")
//     //     )));
//     // }

//     fn get_project_file(dist: &GtlProjectDistOld) -> &GtlProjectFileCompiledOld {
//         dist.files
//             .iter()
//             .find_map(|file| match file {
//                 GtlProjectFileOld::Compiled(file) if file.path.as_str().contains("pyproject.toml") => {
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
