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
    ) -> Option<GtlGenerations<PyProjectModule>> {
        let mut files = vec![];
        let mut diagnostics = None;

        files.push(self.generate_root_init_file(&project.modules));

        let (module_init_files, diagnostic) = self.generate_module_init_files(&project.modules);
        files.extend(module_init_files);
        if let Some(diagnostic) = diagnostic {
            diagnostics = Some(vec![diagnostic]);
        }

        files.push(self.generate_py_typed_file());

        Some((files, diagnostics))
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
        let mut diagnostics = vec![];

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
                            diagnostics.push(GtDiagnostic::error(format!(
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
            diagnostics.push(GtDiagnostic::warning(format!(
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
            diagnostics,
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
    ) -> (Vec<GtlGeneration<PyProjectModule>>, Option<GtDiagnostic>) {
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
                            module_paths.insert(module_parent_path);
                        }
                    }

                    Err(err) => formatted_errors.push(GtDiagnostic::format_report(err)),
                }
            }
        }

        let diagnostic = if formatted_errors.is_empty() {
            None
        } else {
            Some(GtDiagnostic::warning((
                "Some of `__init__.py` could be missing".to_string(),
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

        (files, diagnostic)
    }

    fn py_module_path(&self, module_path: &GtpModulePath) -> Result<String> {
        let module_id = module_path.to_module_id(&self.config.project_paths.src)?;
        Ok(PyPath::to_py_module_path(module_id.as_str_without_ext()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

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
              target_path: "examples/basic/dist/py/module/author.py",
              project_module: PyProjectModule(
                module: PyModule(
                  doc: None,
                  imports: [
                    PyImport(
                      dependency: Runtime,
                      reference: Named([
                        Name(PyIdentifier("Model")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Class(PyClass(
                      doc: None,
                      name: PyIdentifier("Author"),
                      generics: [],
                      extensions: [],
                      properties: [
                        PyProperty(
                          doc: None,
                          name: PyKey("name"),
                          alias: None,
                          descriptor: Primitive(String),
                          required: true,
                        ),
                      ],
                      references: [],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: PyProjectModule(
              module: PyModule(
                doc: None,
                imports: [
                  PyImport(
                    dependency: Runtime,
                    reference: Named([
                      Name(PyIdentifier("Model")),
                    ]),
                  ),
                ],
                definitions: [
                  Class(PyClass(
                    doc: None,
                    name: PyIdentifier("Author"),
                    generics: [],
                    extensions: [],
                    properties: [
                      PyProperty(
                        doc: None,
                        name: PyKey("name"),
                        alias: None,
                        descriptor: Primitive(String),
                        required: true,
                      ),
                    ],
                    references: [],
                  )),
                ],
              ),
            ),
          )),
          "examples/basic/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/basic/src/book.type",
              target_path: "examples/basic/dist/py/module/book.py",
              project_module: PyProjectModule(
                module: PyModule(
                  doc: None,
                  imports: [
                    PyImport(
                      dependency: Local(PyPath(".author")),
                      reference: Named([
                        Name(PyIdentifier("Author")),
                      ]),
                    ),
                    PyImport(
                      dependency: Runtime,
                      reference: Named([
                        Name(PyIdentifier("Model")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Class(PyClass(
                      doc: None,
                      name: PyIdentifier("Book"),
                      generics: [],
                      extensions: [],
                      properties: [
                        PyProperty(
                          doc: None,
                          name: PyKey("title"),
                          alias: None,
                          descriptor: Primitive(String),
                          required: true,
                        ),
                        PyProperty(
                          doc: None,
                          name: PyKey("author"),
                          alias: None,
                          descriptor: Reference(PyReference(
                            identifier: PyIdentifier("Author"),
                            arguments: [],
                            forward: false,
                          )),
                          required: true,
                        ),
                      ],
                      references: [
                        PyIdentifier("Author"),
                      ],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: PyProjectModule(
              module: PyModule(
                doc: None,
                imports: [
                  PyImport(
                    dependency: Local(PyPath(".author")),
                    reference: Named([
                      Name(PyIdentifier("Author")),
                    ]),
                  ),
                  PyImport(
                    dependency: Runtime,
                    reference: Named([
                      Name(PyIdentifier("Model")),
                    ]),
                  ),
                ],
                definitions: [
                  Class(PyClass(
                    doc: None,
                    name: PyIdentifier("Book"),
                    generics: [],
                    extensions: [],
                    properties: [
                      PyProperty(
                        doc: None,
                        name: PyKey("title"),
                        alias: None,
                        descriptor: Primitive(String),
                        required: true,
                      ),
                      PyProperty(
                        doc: None,
                        name: PyKey("author"),
                        alias: None,
                        descriptor: Reference(PyReference(
                          identifier: PyIdentifier("Author"),
                          arguments: [],
                          forward: false,
                        )),
                        required: true,
                      ),
                    ],
                    references: [
                      PyIdentifier("Author"),
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
              target_path: "examples/glob/dist/py/module/author.py",
              project_module: PyProjectModule(
                module: PyModule(
                  doc: None,
                  imports: [
                    PyImport(
                      dependency: Runtime,
                      reference: Named([
                        Name(PyIdentifier("Model")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Alias(PyAlias(
                      doc: None,
                      name: PyIdentifier("AuthorName"),
                      generics: [],
                      descriptor: Primitive(String),
                      references: [],
                    )),
                    Class(PyClass(
                      doc: None,
                      name: PyIdentifier("Author"),
                      generics: [],
                      extensions: [],
                      properties: [
                        PyProperty(
                          doc: None,
                          name: PyKey("name"),
                          alias: None,
                          descriptor: Reference(PyReference(
                            identifier: PyIdentifier("AuthorName"),
                            arguments: [],
                            forward: false,
                          )),
                          required: true,
                        ),
                      ],
                      references: [
                        PyIdentifier("AuthorName"),
                      ],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: PyProjectModule(
              module: PyModule(
                doc: None,
                imports: [
                  PyImport(
                    dependency: Runtime,
                    reference: Named([
                      Name(PyIdentifier("Model")),
                    ]),
                  ),
                ],
                definitions: [
                  Alias(PyAlias(
                    doc: None,
                    name: PyIdentifier("AuthorName"),
                    generics: [],
                    descriptor: Primitive(String),
                    references: [],
                  )),
                  Class(PyClass(
                    doc: None,
                    name: PyIdentifier("Author"),
                    generics: [],
                    extensions: [],
                    properties: [
                      PyProperty(
                        doc: None,
                        name: PyKey("name"),
                        alias: None,
                        descriptor: Reference(PyReference(
                          identifier: PyIdentifier("AuthorName"),
                          arguments: [],
                          forward: false,
                        )),
                        required: true,
                      ),
                    ],
                    references: [
                      PyIdentifier("AuthorName"),
                    ],
                  )),
                ],
              ),
            ),
          )),
          "examples/glob/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/glob/src/book.type",
              target_path: "examples/glob/dist/py/module/book.py",
              project_module: PyProjectModule(
                module: PyModule(
                  doc: None,
                  imports: [
                    PyImport(
                      dependency: Local(PyPath(".author")),
                      reference: Default(Some(PyIdentifier("author"))),
                    ),
                    PyImport(
                      dependency: Pydantic,
                      reference: Named([
                        Name(PyIdentifier("Field")),
                      ]),
                    ),
                    PyImport(
                      dependency: Runtime,
                      reference: Named([
                        Name(PyIdentifier("Model")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Class(PyClass(
                      doc: None,
                      name: PyIdentifier("Book"),
                      generics: [],
                      extensions: [],
                      properties: [
                        PyProperty(
                          doc: None,
                          name: PyKey("title"),
                          alias: None,
                          descriptor: Primitive(String),
                          required: true,
                        ),
                        PyProperty(
                          doc: None,
                          name: PyKey("author"),
                          alias: None,
                          descriptor: Reference(PyReference(
                            identifier: PyIdentifier("author.Author"),
                            arguments: [],
                            forward: false,
                          )),
                          required: true,
                        ),
                        PyProperty(
                          doc: None,
                          name: PyKey("author_name"),
                          alias: Some("authorName"),
                          descriptor: Reference(PyReference(
                            identifier: PyIdentifier("author.AuthorName"),
                            arguments: [],
                            forward: false,
                          )),
                          required: true,
                        ),
                      ],
                      references: [
                        PyIdentifier("author.Author"),
                        PyIdentifier("author.AuthorName"),
                      ],
                    )),
                  ],
                ),
              ),
            ),
            resolved_module: PyProjectModule(
              module: PyModule(
                doc: None,
                imports: [
                  PyImport(
                    dependency: Local(PyPath(".author")),
                    reference: Default(Some(PyIdentifier("author"))),
                  ),
                  PyImport(
                    dependency: Pydantic,
                    reference: Named([
                      Name(PyIdentifier("Field")),
                    ]),
                  ),
                  PyImport(
                    dependency: Runtime,
                    reference: Named([
                      Name(PyIdentifier("Model")),
                    ]),
                  ),
                ],
                definitions: [
                  Class(PyClass(
                    doc: None,
                    name: PyIdentifier("Book"),
                    generics: [],
                    extensions: [],
                    properties: [
                      PyProperty(
                        doc: None,
                        name: PyKey("title"),
                        alias: None,
                        descriptor: Primitive(String),
                        required: true,
                      ),
                      PyProperty(
                        doc: None,
                        name: PyKey("author"),
                        alias: None,
                        descriptor: Reference(PyReference(
                          identifier: PyIdentifier("author.Author"),
                          arguments: [],
                          forward: false,
                        )),
                        required: true,
                      ),
                      PyProperty(
                        doc: None,
                        name: PyKey("author_name"),
                        alias: Some("authorName"),
                        descriptor: Reference(PyReference(
                          identifier: PyIdentifier("author.AuthorName"),
                          arguments: [],
                          forward: false,
                        )),
                        required: true,
                      ),
                    ],
                    references: [
                      PyIdentifier("author.Author"),
                      PyIdentifier("author.AuthorName"),
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
              path: "examples/basic/dist/py/.gitignore",
              source_code: "__pycache__\ndist\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/py/module/__init__.py",
              source_code: "from .author import Author\nfrom .book import Book\n\n\n__all__ = [\"Author\", \"Book\"]",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/py/module/author.py",
              source_code: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Author(Model):\n    name: str\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/py/module/book.py",
              source_code: "from __future__ import annotations\n\n\nfrom .author import Author\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n    author: Author\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/py/module/py.typed",
              source_code: "",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/basic/dist/py/pyproject.toml",
              source_code: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            )),
          ],
          diagnostics: [],
        )
        "#
        );
    }

    #[test]
    fn test_render_nested() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/nested".into(), None).unwrap();

        assert_ron_snapshot!(
          compile(&project),
          @r#"
        GtlDist(
          files: [
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/.gitignore",
              source_code: "__pycache__\ndist\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/module/__init__.py",
              source_code: "from .inventory import Inventory\nfrom .shop.goods.book import Book\n\n\n__all__ = [\"Inventory\", \"Book\"]",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/module/inventory.py",
              source_code: "from __future__ import annotations\n\n\nfrom .shop.goods.book import Book\nfrom genotype import Model\n\n\nclass Inventory(Model):\n    goods: list[Book]\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/module/py.typed",
              source_code: "",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/module/shop/goods/__init__.py",
              source_code: "",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/module/shop/goods/book.py",
              source_code: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/nested/dist/py/pyproject.toml",
              source_code: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            )),
          ],
          diagnostics: [],
        )
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/dependencies".into(), None)
                .unwrap();
        project.config.py.common.dependencies =
            IndexMap::from_iter(vec![("genotype_json_types".into(), "genotype_json".into())]);

        assert_ron_snapshot!(
          compile(&project),
          @r#"
        GtlDist(
          files: [
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/py/.gitignore",
              source_code: "__pycache__\ndist\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/py/module/__init__.py",
              source_code: "from .prompt import Prompt\n\n\n__all__ = [\"Prompt\"]",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/py/module/prompt.py",
              source_code: "from __future__ import annotations\n\n\nfrom genotype_json import JsonAny\nfrom genotype import Model\n\n\nclass Prompt(Model):\n    content: str\n    output: JsonAny\n",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/py/module/py.typed",
              source_code: "",
            )),
            Generated(GtlDistFileGenerated(
              path: "examples/dependencies/dist/py/pyproject.toml",
              source_code: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            )),
          ],
          diagnostics: [],
        )
        "#
        );
    }

    #[test]
    fn test_render_cyclic_lists() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/cyclic-lists".into(), None)
                .unwrap();

        let dist = compile(&project);

        let json = get_dist_file(&dist, "module/json.py");
        assert_snapshot!(
            json.source_code,
            @r#"
        from __future__ import annotations


        from typing import Optional, Literal
        from genotype import Model


        class JsonBase(Model):
            name: Optional[str] = None
            doc: Optional[str] = None


        class JsonNull(JsonBase, Model):
            kind: Literal["null"]


        class JsonBoolean(JsonBase, Model):
            kind: Literal["boolean"]


        class JsonNumber(JsonBase, Model):
            kind: Literal["number"]


        class JsonString(JsonBase, Model):
            kind: Literal["string"]


        class JsonLiteral(JsonBase, Model):
            kind: Literal["literal"]
            value: str | float | bool | Literal[None]


        type JsonLiteralKind = Literal["string"] | Literal["number"] | Literal["boolean"] | Literal["null"]


        class JsonTuple(JsonBase, Model):
            kind: Literal["tuple"]
            descriptors: list[JsonAny]


        class JsonUnion(JsonBase, Model):
            kind: Literal["union"]
            descriptors: list[JsonAny]


        class JsonProperty(Model):
            kind: Literal["property"]
            name: str
            doc: Optional[str] = None
            descriptor: JsonAny
            required: Optional[bool] = None


        class JsonObject(JsonBase, Model):
            kind: Literal["object"]
            properties: list[JsonProperty]


        class JsonArray(JsonBase, Model):
            kind: Literal["array"]
            descriptor: JsonAny


        type JsonAny = JsonNull | JsonBoolean | JsonNumber | JsonString | JsonArray | JsonObject | JsonUnion | JsonLiteral | JsonTuple
        "#
        );

        let init = get_dist_file(&dist, "module/__init__.py");
        assert_snapshot!(
            init.source_code,
            @r#"
        from .json import JsonBase, JsonNull, JsonBoolean, JsonNumber, JsonString, JsonLiteral, JsonLiteralKind, JsonTuple, JsonUnion, JsonProperty, JsonObject, JsonArray, JsonAny


        __all__ = ["JsonBase", "JsonNull", "JsonBoolean", "JsonNumber", "JsonString", "JsonLiteral", "JsonLiteralKind", "JsonTuple", "JsonUnion", "JsonProperty", "JsonObject", "JsonArray", "JsonAny"]
        "#
        );
    }

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.version = Some("0.2.0".parse().unwrap());

        let dist = compile(&project);
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source_code,
            @r#"
        [tool.poetry]
        packages = [{ include = "module" }]
        version = "0.2.0"

        [tool.poetry.dependencies]
        python = "^3.13"
        genotype-runtime = "^0.4"

        [build-system]
        requires = ["poetry-core"]
        build-backend = "poetry.core.masonry.api"
        "#
        );
    }

    #[test]
    fn test_render_prefers_py_manifest_version_over_global() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.version = Some("0.2.0".parse().unwrap());
        project.config.py.common.manifest = toml::from_str(
            r#"[tool.poetry]
version = "0.3.0"
"#,
        )
        .unwrap();

        let dist = compile(&project);
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source_code,
            @r#"
        [tool.poetry]
        packages = [{ include = "module" }]
        version = "0.3.0"

        [tool.poetry.dependencies]
        python = "^3.13"
        genotype-runtime = "^0.4"

        [build-system]
        requires = ["poetry-core"]
        build-backend = "poetry.core.masonry.api"
        "#
        );
    }

    #[test]
    fn test_render_uv_manifest() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.py.lang.manager = genotype_lang_py_config::PyPackageManager::Uv;
        project.config.version = Some("0.2.0".parse().unwrap());
        project.config.py.common.manifest = toml::from_str(
            r#"[project]
name = "module"
"#,
        )
        .unwrap();

        let dist = compile(&project);
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source_code,
            @r#"
        [project]
        requires-python = ">=3.13,<4"
        version = "0.2.0"
        name = "module"
        dependencies = ["genotype-runtime>=0.4,<0.5"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.hatch.build.targets.wheel]
        packages = ["module"]
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
          "examples/basic/dist/py/__init__.py",
          "examples/basic/dist/py/author.py",
          "examples/basic/dist/py/book.py",
          "examples/basic/dist/py/py.typed",
        ]
        "#
        );
    }

    #[test]
    fn test_render_without_package_target() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config.package = true;
        project.config.py.common.package = Some(false);

        let dist = compile(&project);

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/py/__init__.py",
          "examples/basic/dist/py/author.py",
          "examples/basic/dist/py/book.py",
          "examples/basic/dist/py/py.typed",
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
                if error.path.as_str() == "examples/basic/dist/py/module/broken.py"
                    && error.message.contains("Failed to convert")
        )));
    }

    fn modules(project: &GtProject) -> GtlProjectModules<PyProjectModule> {
        let compiler = PyCompiler::new(project);
        let mut lang_project = GtlProject::<PyProjectModule>::new(compiler.config());
        lang_project.convert(&project.modules);
        lang_project.resolve().unwrap();
        lang_project.modules
    }

    fn compile(project: &GtProject) -> GtlDist {
        PyCompiler::new(project).compile().unwrap().unwrap()
    }

    fn get_project_file(dist: &GtlDist) -> &GtlDistFileGenerated {
        dist.files
            .iter()
            .find_map(|file| match file {
                GtlDistFile::Generated(file) if file.path.as_str().contains("pyproject.toml") => {
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
