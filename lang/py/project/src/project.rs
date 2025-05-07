use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct PyProject<'a> {
    pub modules: Vec<PyProjectModule>,
    project: &'a GTProject,
}

impl<'a> GtlProject<'a> for PyProject<'a> {
    type Module = PyProjectModule;

    fn generate(project: &'a GTProject) -> Result<Self> {
        let modules = project
            .modules
            .iter()
            .map(|module| PyProjectModule::generate(&project, module))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, project })
    }

    fn out(&self) -> Result<GtlProjectOut> {
        let gitignore = GtlProjectFile {
            path: self.project.config.py.package_path(".gitignore".into()),
            source: r#"__pycache__
dist"#
                .into(),
        };

        let pyproject =
            PyProjectManifest::manifest_file(&self.project.config.py, &self.dependencies())?;

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
            path: self.project.config.py.src_file_path("__init__.py".into()),
            source: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let py_typed = GtlProjectFile {
            path: self.project.config.py.src_file_path("py.typed".into()),
            source: "".into(),
        };

        let module_root = self.project.config.py.src_dir_path();
        let mut module_paths: HashSet<PathBuf> = HashSet::new();

        for module in self.modules.iter() {
            // [TODo]
            let module_path = module.path.parent().unwrap();
            if module_root != module_path {
                module_paths.insert(module_path.into());
            }
        }

        let module_inits = module_paths.into_iter().map(|module_path| GtlProjectFile {
            path: module_path.join("__init__.py"),
            source: "".into(),
        });

        let mut render_context = PYRenderContext {
            config: &self.project.config.py.lang,
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
                        path: module.path.clone(),
                        source: code,
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut modules = vec![gitignore, pyproject, py_typed, init];
        modules.extend(module_inits);
        modules.extend(project_modules);

        Ok(GtlProjectOut { files: modules })
    }

    fn modules(&self) -> Vec<Self::Module> {
        self.modules.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_config::GtConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        let config = GtConfig::from_root("module", "./examples/basic");
        let project = GTProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().modules,
            vec![
                PyProjectModule {
                    name: "author".into(),
                    path: "libs/py/module/author.py".into(),
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
                    path: "libs/py/module/book.py".into(),
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
        let project = GTProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().modules,
            vec![
                PyProjectModule {
                    name: "author".into(),
                    path: "libs/py/module/author.py".into(),
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
                    path: "libs/py/module/book.py".into(),
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
        let project = GTProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().out().unwrap(),
            GtlProjectOut {
                files: vec![
                    GtlProjectFile {
                        path: "libs/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/pyproject.toml".into(),
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
                        path: "libs/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/__init__.py".into(),
                        source: r#"from .author import Author
from .book import Book


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/author.py".into(),
                        source: r#"from genotype import Model


class Author(Model):
    name: str
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/py/module/book.py".into(),
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
        let project = GTProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().out().unwrap(),
            GtlProjectOut {
                files: vec![
                    GtlProjectFile {
                        path: "libs/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/pyproject.toml".into(),
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
                        path: "libs/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/__init__.py".into(),
                        source: r#"from .inventory import Inventory
from .shop.goods.book import Book


__all__ = ["Inventory", "Book"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/shop/goods/__init__.py".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/inventory.py".into(),
                        source: r#"from .shop.goods.book import Book
from genotype import Model


class Inventory(Model):
    goods: list[Book]
"#
                        .into()
                    },
                    GtlProjectFile {
                        path: "libs/py/module/shop/goods/book.py".into(),
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

        let project = GTProject::load(config).unwrap();

        assert_eq!(
            PyProject::generate(&project).unwrap().out().unwrap(),
            GtlProjectOut {
                files: vec![
                    GtlProjectFile {
                        path: "libs/py/.gitignore".into(),
                        source: r#"__pycache__
dist"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/pyproject.toml".into(),
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
                        path: "libs/py/module/py.typed".into(),
                        source: "".into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/__init__.py".into(),
                        source: r#"from .prompt import Prompt


__all__ = ["Prompt"]"#
                            .into(),
                    },
                    GtlProjectFile {
                        path: "libs/py/module/prompt.py".into(),
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
