use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
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

        // let module_root = self.config.pkg_src_path();
        let mut module_paths: HashSet<GtPkgSrcRelativePath> = HashSet::new();

        for module in self.modules.iter() {
            // [TODo]
            if let Some(module_path) = module.path.parent() {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().modules,
            vec![
                PyProjectModule {
                    name: "author".into(),
                    path: "author.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![PYImport {
                            reference: PYImportReference::Named(vec![PYImportName::Name(
                                "Model".into()
                            )]),
                            dependency: PYDependencyIdent::Runtime,
                        }],
                        definitions: vec![PYDefinition::Class(PYClass {
                            doc: None,
                            name: "Author".into(),
                            extensions: vec![],
                            properties: vec![PYProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true,
                            }],
                            references: vec![],
                        })]
                    },
                },
                PyProjectModule {
                    name: "book".into(),
                    path: "book.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![
                            PYImport {
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Author".into()
                                )]),
                                dependency: PYDependencyIdent::Path(".author".into()),
                            },
                            PYImport {
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                                dependency: PYDependencyIdent::Runtime,
                            }
                        ],
                        definitions: vec![PYDefinition::Class(PYClass {
                            doc: None,
                            name: "Book".into(),
                            extensions: vec![],
                            properties: vec![
                                PYProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                    required: true,
                                },
                                PYProperty {
                                    doc: None,
                                    name: "author".into(),
                                    descriptor: PYReference::new("Author".into(), false).into(),
                                    required: true,
                                },
                            ],
                            references: vec![PYIdentifier("Author".into()),],
                        })],
                    },
                },
            ]
        )
    }

    #[test]
    fn test_convert_glob() {
        let config = GtConfig::from_root("module", "./examples/glob");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().modules,
            vec![
                PyProjectModule {
                    name: "author".into(),
                    path: "author.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![PYImport {
                            reference: PYImportReference::Named(vec![PYImportName::Name(
                                "Model".into()
                            )]),
                            dependency: PYDependencyIdent::Runtime,
                        }],
                        definitions: vec![
                            PYDefinition::Alias(PYAlias {
                                doc: None,
                                name: "AuthorName".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                references: vec![],
                            }),
                            PYDefinition::Class(PYClass {
                                doc: None,
                                name: "Author".into(),
                                extensions: vec![],
                                properties: vec![PYProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: PYReference::new("AuthorName".into(), false).into(),
                                    required: true,
                                }],
                                references: vec![PYIdentifier("AuthorName".into()),],
                            }),
                        ]
                    },
                },
                PyProjectModule {
                    name: "book".into(),
                    path: "book.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![
                            PYImport {
                                reference: PYImportReference::Default(Some("author".into())),
                                dependency: PYDependencyIdent::Path(".author".into()),
                            },
                            PYImport {
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                                dependency: PYDependencyIdent::Runtime,
                            }
                        ],
                        definitions: vec![PYDefinition::Class(PYClass {
                            doc: None,
                            name: "Book".into(),
                            extensions: vec![],
                            properties: vec![
                                PYProperty {
                                    doc: None,
                                    name: "title".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                    required: true,
                                },
                                PYProperty {
                                    doc: None,
                                    name: "author".into(),
                                    descriptor: PYReference::new("author.Author".into(), false)
                                        .into(),
                                    required: true,
                                },
                                PYProperty {
                                    doc: None,
                                    name: "author_name".into(),
                                    descriptor: PYReference::new("author.AuthorName".into(), false)
                                        .into(),
                                    required: true,
                                },
                            ],
                            references: vec![
                                PYIdentifier("author.Author".into()),
                                PYIdentifier("author.AuthorName".into()),
                            ],
                        })],
                    },
                },
            ]
        )
    }

    #[test]
    fn test_render() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/basic/dist/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "dist/py/pyproject.toml".into(),
                        source: r#"[tool.poetry]
packages = [{ include = "module" }]

[tool.poetry.dependencies]
python = "^3.12"
genotype-runtime = "^0.4"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/py/module/__init__.py".into(),
                        source: r#"from .author import Author
from .book import Book


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/py/module/author.py".into(),
                        source: r#"from genotype import Model


class Author(Model):
    name: str
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/basic/dist/py/module/book.py".into(),
                        source: r#"from .author import Author
from genotype import Model


class Book(Model):
    title: str
    author: Author
"#
                        .into()
                    }
                ]
            }
        )
    }

    #[test]
    fn test_render_nested() {
        let config = GtConfig::from_root("module", "./examples/nested");
        let project = GtProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/nested/dist/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/pyproject.toml".into(),
                        source: r#"[tool.poetry]
packages = [{ include = "module" }]

[tool.poetry.dependencies]
python = "^3.12"
genotype-runtime = "^0.4"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/module/__init__.py".into(),
                        source: r#"from .inventory import Inventory
from .shop.goods.book import Book


__all__ = ["Inventory", "Book"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/module/shop/goods/__init__.py".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/module/inventory.py".into(),
                        source: r#"from .shop.goods.book import Book
from genotype import Model


class Inventory(Model):
    goods: list[Book]
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "examples/nested/dist/py/module/shop/goods/book.py".into(),
                        source: r#"from genotype import Model


class Book(Model):
    title: str
"#
                        .into()
                    }
                ]
            }
        )
    }

    #[test]
    fn test_render_dependencies() {
        let mut config = GtConfig::from_root("module", "./examples/dependencies");
        config.py.common.dependencies =
            HashMap::from_iter(vec![("genotype_json_types".into(), "genotype_json".into())]);

        let project = GtProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().dist().unwrap(),
            GtlProjectDist {
                files: vec![
                    GtlProjectFile {
                        path: "examples/dependencies/dist/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/py/pyproject.toml".into(),
                        source: r#"[tool.poetry]
packages = [{ include = "module" }]

[tool.poetry.dependencies]
python = "^3.12"
genotype-runtime = "^0.4"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                        .into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/py/module/__init__.py".into(),
                        source: r#"from .prompt import Prompt


__all__ = ["Prompt"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "examples/dependencies/dist/py/module/prompt.py".into(),
                        source: r#"from genotype_json import JsonAny
from genotype import Model


class Prompt(Model):
    content: str
    output: JsonAny
"#
                        .into()
                    },
                ]
            }
        )
    }
}
