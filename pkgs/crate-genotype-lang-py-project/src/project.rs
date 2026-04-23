use serde::Serialize;

use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PyProject<'a> {
    pub modules: Vec<PyProjectModule>,
    pub config: GtpConfigPkg<'a, PyConfig>,
}

impl<'a> GtlProject<'a> for PyProject<'a> {
    type Module = PyProjectModule;

    type LangConfig = PyConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.config.pkg_config_py();
        let modules = project
            .modules_legacy
            .iter()
            .map(|module| PyProjectModule::generate(config.target, module))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, config })
    }

    fn dist(&self) -> Result<GtlProjectDist> {
        let gitignore = GtlProjectFile {
            path: self.config.pkg_file_path(&".gitignore".into()),
            source: r#"__pycache__
dist"#
                .into(),
        };

        let pyproject = self.generate_manifest(&self.dependencies())?;

        let mut imports = vec![];
        let mut exports = vec![];
        for module in self.modules.iter() {
            let mut definitions = vec![];
            for definition in module.module.definitions.iter() {
                let name = definition.name();
                definitions.push(name.0.clone());
                exports.push(format!("\"{}\"", name.0.clone()));
            }

            imports.push(format!(
                "from .{} import {}",
                module.name.clone(),
                definitions.join(", ")
            ));
        }

        let init = GtlProjectFile {
            path: self.config.pkg_src_file_path(&"__init__.py".into()),
            source: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let py_typed = GtlProjectFile {
            path: self.config.pkg_src_file_path(&"py.typed".into()),
            source: "".into(),
        };

        let mut module_paths: HashSet<GtpPkgSrcDirRelativePath> = HashSet::new();

        for module in self.modules.iter() {
            // [TODo]
            if let Some(module_path) = module.path.to_parent() {
                if module_path == ".".into() {
                    continue;
                }

                module_paths.insert(module_path);
            }
        }

        let module_inits = module_paths.into_iter().map(|module_path| GtlProjectFile {
            path: self
                .config
                .pkg_src_file_path(&module_path.join_relative_path(&"__init__.py".into())),
            source: "".into(),
        });

        let mut render_context = PyRenderContext {
            config: &self.config.target.lang,
        };

        let project_modules = self
            .modules
            .iter()
            .map(|module| {
                module
                    .module
                    .render(Default::default(), &mut render_context)
                    .map(|code| GtlProjectFile {
                        path: self.config.pkg_src_file_path(&module.path),
                        source: code,
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut modules = vec![gitignore, pyproject, py_typed, init];
        modules.extend(module_inits);
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
    use genotype_test::*;

    #[test]
    fn test_convert_base() {
        let config = GtpConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().modules,
          @r#"
        [
          PyProjectModule(
            name: "author",
            path: "author.py",
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
                  extensions: [],
                  properties: [
                    PyProperty(
                      doc: None,
                      name: PyKey("name"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                  ],
                  references: [],
                )),
              ],
            ),
          ),
          PyProjectModule(
            name: "book",
            path: "book.py",
            module: PyModule(
              doc: None,
              imports: [
                PyImport(
                  dependency: Path(PyPath(".author")),
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
                  extensions: [],
                  properties: [
                    PyProperty(
                      doc: None,
                      name: PyKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    PyProperty(
                      doc: None,
                      name: PyKey("author"),
                      descriptor: Reference(PyReference(
                        identifier: PyIdentifier("Author"),
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
        ]
        "#
        );
    }

    #[test]
    fn test_convert_glob() {
        let config = GtpConfig::from_root("module", "./examples/glob");
        let project = GtProject::load(config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().modules,
          @r#"
        [
          PyProjectModule(
            name: "author",
            path: "author.py",
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
                  descriptor: Primitive(String),
                  references: [],
                )),
                Class(PyClass(
                  doc: None,
                  name: PyIdentifier("Author"),
                  extensions: [],
                  properties: [
                    PyProperty(
                      doc: None,
                      name: PyKey("name"),
                      descriptor: Reference(PyReference(
                        identifier: PyIdentifier("AuthorName"),
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
          PyProjectModule(
            name: "book",
            path: "book.py",
            module: PyModule(
              doc: None,
              imports: [
                PyImport(
                  dependency: Path(PyPath(".author")),
                  reference: Default(Some(PyIdentifier("author"))),
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
                  extensions: [],
                  properties: [
                    PyProperty(
                      doc: None,
                      name: PyKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    PyProperty(
                      doc: None,
                      name: PyKey("author"),
                      descriptor: Reference(PyReference(
                        identifier: PyIdentifier("author.Author"),
                        forward: false,
                      )),
                      required: true,
                    ),
                    PyProperty(
                      doc: None,
                      name: PyKey("author_name"),
                      descriptor: Reference(PyReference(
                        identifier: PyIdentifier("author.AuthorName"),
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
        ]
        "#
        );
    }

    #[test]
    fn test_render() {
        let config = GtpConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/basic/dist/py/.gitignore",
              source: "__pycache__\ndist",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/pyproject.toml",
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/module/py.typed",
              source: "",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/module/__init__.py",
              source: "from .author import Author\nfrom .book import Book\n\n\n__all__ = [\"Author\", \"Book\"]",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/module/author.py",
              source: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Author(Model):\n    name: str\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/module/book.py",
              source: "from __future__ import annotations\n\n\nfrom .author import Author\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n    author: Author\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_nested() {
        let config = GtpConfig::from_root("module", "./examples/nested");
        let project = GtProject::load(config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/nested/dist/py/.gitignore",
              source: "__pycache__\ndist",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/pyproject.toml",
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/py.typed",
              source: "",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/__init__.py",
              source: "from .inventory import Inventory\nfrom .shop.goods.book import Book\n\n\n__all__ = [\"Inventory\", \"Book\"]",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/shop/goods/__init__.py",
              source: "",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/inventory.py",
              source: "from __future__ import annotations\n\n\nfrom .shop.goods.book import Book\nfrom genotype import Model\n\n\nclass Inventory(Model):\n    goods: list[Book]\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/shop/goods/book.py",
              source: "from __future__ import annotations\n\n\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let mut config = GtpConfig::from_root("module", "./examples/dependencies");
        config.py.common.dependencies =
            HashMap::from_iter(vec![("genotype_json_types".into(), "genotype_json".into())]);

        let project = GtProject::load(config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().dist().unwrap(),
          @r#"
        GtlProjectDist(
          files: [
            GtlProjectFile(
              path: "examples/dependencies/dist/py/.gitignore",
              source: "__pycache__\ndist",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/py/pyproject.toml",
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.13\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/py/module/py.typed",
              source: "",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/py/module/__init__.py",
              source: "from .prompt import Prompt\n\n\n__all__ = [\"Prompt\"]",
            ),
            GtlProjectFile(
              path: "examples/dependencies/dist/py/module/prompt.py",
              source: "from __future__ import annotations\n\n\nfrom genotype_json import JsonAny\nfrom genotype import Model\n\n\nclass Prompt(Model):\n    content: str\n    output: JsonAny\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_cyclic_lists() {
        let config = GtpConfig::from_root("module", "./examples/cyclic-lists");
        let project = GtProject::load(config).unwrap();

        let dist = PyProject::generate(&project).unwrap().dist().unwrap();

        let json = get_dist_file(&dist, "module/json.py");
        assert_snapshot!(
            json.source,
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
            init.source,
            @r#"
        from .json import JsonBase, JsonNull, JsonBoolean, JsonNumber, JsonString, JsonLiteral, JsonLiteralKind, JsonTuple, JsonUnion, JsonProperty, JsonObject, JsonArray, JsonAny


        __all__ = ["JsonBase", "JsonNull", "JsonBoolean", "JsonNumber", "JsonString", "JsonLiteral", "JsonLiteralKind", "JsonTuple", "JsonUnion", "JsonProperty", "JsonObject", "JsonArray", "JsonAny"]
        "#
        );
    }

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut config = GtpConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());
        let project = GtProject::load(config).unwrap();

        let dist = PyProject::generate(&project).unwrap().dist().unwrap();
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source,
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
        let mut config = GtpConfig::from_root("module", "./examples/basic");
        config.version = Some("0.2.0".parse().unwrap());
        config.py.common.manifest = toml::from_str(
            r#"[tool.poetry]
version = "0.3.0"
"#,
        )
        .unwrap();

        let project = GtProject::load(config).unwrap();

        let dist = PyProject::generate(&project).unwrap().dist().unwrap();
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source,
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
        let mut config = GtpConfig::from_root("module", "./examples/basic");
        config.py.lang.manager = genotype_lang_py_config::PyPackageManager::Uv;
        config.version = Some("0.2.0".parse().unwrap());
        config.py.common.manifest = toml::from_str(
            r#"[project]
name = "module"
"#,
        )
        .unwrap();

        let project = GtProject::load(config).unwrap();

        let dist = PyProject::generate(&project).unwrap().dist().unwrap();
        let pyproject = get_project_file(&dist);

        assert_snapshot!(
            pyproject.source,
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

    fn get_project_file(dist: &GtlProjectDist) -> &GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().contains("pyproject.toml"))
            .unwrap()
    }

    fn get_dist_file<'a>(dist: &'a GtlProjectDist, path_suffix: &str) -> &'a GtlProjectFile {
        dist.files
            .iter()
            .find(|file| file.path.as_str().ends_with(path_suffix))
            .unwrap()
    }
}
