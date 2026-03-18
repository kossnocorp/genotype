use serde::Serialize;

use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PyProject<'a> {
    pub modules: Vec<PyProjectModule>,
    pub config: GtConfigPkg<'a, PyConfig>,
}

impl<'a> GtlProject<'a> for PyProject<'a> {
    type Module = PyProjectModule;

    type LangConfig = PyConfig;

    fn generate(project: &'a GtProject) -> Result<Self> {
        let config = project.config.pkg_config_py();
        let modules = project
            .modules
            .iter()
            .map(|module| PyProjectModule::generate(&config.target, module))
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

        let mut module_paths: HashSet<GtPkgSrcRelativePath> = HashSet::new();

        for module in self.modules.iter() {
            // [TODo]
            if let Some(module_path) = module.path.parent() {
                if module_path == ".".into() {
                    continue;
                }

                module_paths.insert(module_path);
            }
        }

        let module_inits = module_paths.into_iter().map(|module_path| GtlProjectFile {
            path: self
                .config
                .pkg_src_file_path(&module_path.join_path(&"__init__.py".into())),
            source: "".into(),
        });

        let mut render_context = PYRenderContext {
            config: &self.config.target.lang,
            ..Default::default()
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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(&config).unwrap();

        assert_ron_snapshot!(
          PyProject::generate(&project).unwrap().modules,
          @r#"
        [
          PyProjectModule(
            name: "author",
            path: "author.py",
            module: PYModule(
              doc: None,
              imports: [
                PYImport(
                  dependency: Runtime,
                  reference: Named([
                    Name(PYIdentifier("Model")),
                  ]),
                ),
              ],
              definitions: [
                Class(PYClass(
                  doc: None,
                  name: PYIdentifier("Author"),
                  extensions: [],
                  properties: [
                    PYProperty(
                      doc: None,
                      name: PYKey("name"),
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
            module: PYModule(
              doc: None,
              imports: [
                PYImport(
                  dependency: Path(PYPath(".author")),
                  reference: Named([
                    Name(PYIdentifier("Author")),
                  ]),
                ),
                PYImport(
                  dependency: Runtime,
                  reference: Named([
                    Name(PYIdentifier("Model")),
                  ]),
                ),
              ],
              definitions: [
                Class(PYClass(
                  doc: None,
                  name: PYIdentifier("Book"),
                  extensions: [],
                  properties: [
                    PYProperty(
                      doc: None,
                      name: PYKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    PYProperty(
                      doc: None,
                      name: PYKey("author"),
                      descriptor: Reference(PYReference(
                        identifier: PYIdentifier("Author"),
                        forward: false,
                      )),
                      required: true,
                    ),
                  ],
                  references: [
                    PYIdentifier("Author"),
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
          PyProject::generate(&project).unwrap().modules,
          @r#"
        [
          PyProjectModule(
            name: "author",
            path: "author.py",
            module: PYModule(
              doc: None,
              imports: [
                PYImport(
                  dependency: Runtime,
                  reference: Named([
                    Name(PYIdentifier("Model")),
                  ]),
                ),
              ],
              definitions: [
                Alias(PYAlias(
                  doc: None,
                  name: PYIdentifier("AuthorName"),
                  descriptor: Primitive(String),
                  references: [],
                )),
                Class(PYClass(
                  doc: None,
                  name: PYIdentifier("Author"),
                  extensions: [],
                  properties: [
                    PYProperty(
                      doc: None,
                      name: PYKey("name"),
                      descriptor: Reference(PYReference(
                        identifier: PYIdentifier("AuthorName"),
                        forward: false,
                      )),
                      required: true,
                    ),
                  ],
                  references: [
                    PYIdentifier("AuthorName"),
                  ],
                )),
              ],
            ),
          ),
          PyProjectModule(
            name: "book",
            path: "book.py",
            module: PYModule(
              doc: None,
              imports: [
                PYImport(
                  dependency: Path(PYPath(".author")),
                  reference: Default(Some(PYIdentifier("author"))),
                ),
                PYImport(
                  dependency: Runtime,
                  reference: Named([
                    Name(PYIdentifier("Model")),
                  ]),
                ),
              ],
              definitions: [
                Class(PYClass(
                  doc: None,
                  name: PYIdentifier("Book"),
                  extensions: [],
                  properties: [
                    PYProperty(
                      doc: None,
                      name: PYKey("title"),
                      descriptor: Primitive(String),
                      required: true,
                    ),
                    PYProperty(
                      doc: None,
                      name: PYKey("author"),
                      descriptor: Reference(PYReference(
                        identifier: PYIdentifier("author.Author"),
                        forward: false,
                      )),
                      required: true,
                    ),
                    PYProperty(
                      doc: None,
                      name: PYKey("author_name"),
                      descriptor: Reference(PYReference(
                        identifier: PYIdentifier("author.AuthorName"),
                        forward: false,
                      )),
                      required: true,
                    ),
                  ],
                  references: [
                    PYIdentifier("author.Author"),
                    PYIdentifier("author.AuthorName"),
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
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.12\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
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
              source: "from genotype import Model\n\n\nclass Author(Model):\n    name: str\n",
            ),
            GtlProjectFile(
              path: "examples/basic/dist/py/module/book.py",
              source: "from .author import Author\nfrom genotype import Model\n\n\nclass Book(Model):\n    title: str\n    author: Author\n",
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
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.12\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
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
              source: "from .shop.goods.book import Book\nfrom genotype import Model\n\n\nclass Inventory(Model):\n    goods: list[Book]\n",
            ),
            GtlProjectFile(
              path: "examples/nested/dist/py/module/shop/goods/book.py",
              source: "from genotype import Model\n\n\nclass Book(Model):\n    title: str\n",
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let mut config = GtConfig::from_root("module", "./examples/dependencies");
        config.py.common.dependencies =
            HashMap::from_iter(vec![("genotype_json_types".into(), "genotype_json".into())]);

        let project = GtProject::load(&config).unwrap();

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
              source: "[tool.poetry]\npackages = [{ include = \"module\" }]\n\n[tool.poetry.dependencies]\npython = \"^3.12\"\ngenotype-runtime = \"^0.4\"\n\n[build-system]\nrequires = [\"poetry-core\"]\nbuild-backend = \"poetry.core.masonry.api\"\n",
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
              source: "from genotype_json import JsonAny\nfrom genotype import Model\n\n\nclass Prompt(Model):\n    content: str\n    output: JsonAny\n",
            ),
          ],
        )
        "#
        );
    }
}
