use std::{collections::HashSet, path::PathBuf};

use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
    source::GTLangProjectSource,
};
use genotype_lang_py_config::PYProjectConfig;
use genotype_lang_py_tree::{py_indent, PYRender};
use genotype_project::project::GTProject;
use miette::Result;

use crate::module::PYProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProject {
    pub modules: Vec<PYProjectModule>,
    config: PYProjectConfig,
}

impl GTLangProject<PYProjectConfig> for PYProject {
    fn generate(project: &GTProject, config: PYProjectConfig) -> Result<Self> {
        let modules = project
            .modules
            .iter()
            .map(|module| PYProjectModule::generate(&project, module, &config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules, config })
    }

    fn render(&self) -> Result<GTLangProjectRender> {
        let gitignore = GTLangProjectSource {
            path: self.config.package_path(".gitignore".into()),
            code: r#"__pycache__
dist"#
                .into(),
        };

        let dependencies = self
            .modules
            .iter()
            .flat_map(|module| {
                module
                    .module
                    .imports
                    .iter()
                    .map(|import| import.dependency.clone())
            })
            .collect::<HashSet<_>>();

        let pyproject = GTLangProjectSource {
            path: self.config.package_path("pyproject.toml".into()),
            code: format!(
                r#"[tool.poetry]{}
packages = [{{ include = "{}" }}]

[tool.poetry.dependencies]
{}{}

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#,
                if let Some(package) = &self.config.package {
                    format!("\n{}", package)
                } else {
                    "".into()
                },
                self.config.module,
                self.config.lang.version.as_dependency_str(),
                dependencies.iter().fold("".into(), |acc, dependency| {
                    if let Some(str) = dependency.external_str() {
                        format!("{acc}\n{str}")
                    } else {
                        acc
                    }
                })
            ),
        };

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

        let init = GTLangProjectSource {
            path: self.config.source_path("__init__.py".into()),
            code: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let py_typed = GTLangProjectSource {
            path: self.config.source_path("py.typed".into()),
            code: "".into(),
        };

        let module_root = self.config.module_root_path();
        let mut module_paths: HashSet<PathBuf> = HashSet::new();

        for module in self.modules.iter() {
            // [TODo]
            let module_path = module.path.parent().unwrap();
            if module_root != module_path {
                module_paths.insert(module_path.into());
            }
        }

        let module_inits = module_paths
            .into_iter()
            .map(|module_path| GTLangProjectSource {
                path: module_path.join("__init__.py"),
                code: "".into(),
            });

        let project_modules = self
            .modules
            .iter()
            .map(|module| GTLangProjectSource {
                path: module.path.clone(),
                code: module.module.render(&py_indent(), &self.config.lang),
            })
            .collect::<Vec<_>>();

        let mut modules = vec![gitignore, pyproject, py_typed, init];
        modules.extend(module_inits);
        modules.extend(project_modules);

        Ok(GTLangProjectRender { files: modules })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use genotype_config::GTConfig;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, py_config).unwrap().modules,
            vec![
                PYProjectModule {
                    name: "author".into(),
                    path: "libs/py/module/author.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![PYImport {
                            path: "genotype".into(),
                            reference: PYImportReference::Named(vec![PYImportName::Name(
                                "Model".into()
                            )]),
                            dependency: PYDependency::Runtime,
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
                PYProjectModule {
                    name: "book".into(),
                    path: "libs/py/module/book.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![
                            PYImport {
                                path: ".author".into(),
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Author".into()
                                )]),
                                dependency: PYDependency::Local(".author".into()),
                            },
                            PYImport {
                                path: "genotype".into(),
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                                dependency: PYDependency::Runtime,
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
        let config = GTConfig::from_root("module", "./examples/glob");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, py_config).unwrap().modules,
            vec![
                PYProjectModule {
                    name: "author".into(),
                    path: "libs/py/module/author.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![PYImport {
                            path: "genotype".into(),
                            reference: PYImportReference::Named(vec![PYImportName::Name(
                                "Model".into()
                            )]),
                            dependency: PYDependency::Runtime,
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
                PYProjectModule {
                    name: "book".into(),
                    path: "libs/py/module/book.py".into(),
                    module: PYModule {
                        doc: None,
                        imports: vec![
                            PYImport {
                                path: ".author".into(),
                                reference: PYImportReference::Default(Some("author".into())),
                                dependency: PYDependency::Local(".author".into()),
                            },
                            PYImport {
                                path: "genotype".into(),
                                reference: PYImportReference::Named(vec![PYImportName::Name(
                                    "Model".into()
                                )]),
                                dependency: PYDependency::Runtime,
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
        let config = GTConfig::from_root("module", "./examples/basic");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, py_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/py/.gitignore".into(),
                        code: r#"__pycache__
dist"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/pyproject.toml".into(),
                        code: r#"[tool.poetry]
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
                    GTLangProjectSource {
                        path: "libs/py/module/py.typed".into(),
                        code: "".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/__init__.py".into(),
                        code: r#"from .author import Author
from .book import Book


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/author.py".into(),
                        code: r#"from genotype import Model


class Author(Model):
    name: str
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/book.py".into(),
                        code: r#"from .author import Author
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
        let config = GTConfig::from_root("module", "./examples/nested");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, py_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/py/.gitignore".into(),
                        code: r#"__pycache__
dist"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/pyproject.toml".into(),
                        code: r#"[tool.poetry]
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
                    GTLangProjectSource {
                        path: "libs/py/module/py.typed".into(),
                        code: "".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/__init__.py".into(),
                        code: r#"from .inventory import Inventory
from .shop.goods.book import Book


__all__ = ["Inventory", "Book"]"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/shop/goods/__init__.py".into(),
                        code: "".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/inventory.py".into(),
                        code: r#"from .shop.goods.book import Book
from genotype import Model


class Inventory(Model):
    goods: list[Book]
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/shop/goods/book.py".into(),
                        code: r#"from genotype import Model


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
        let config = GTConfig::from_root("module", "./examples/dependencies");
        let mut py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        py_config.dependencies = Some(HashMap::from_iter(vec![(
            "genotype_json_schema".into(),
            "genotype_json".into(),
        )]));

        assert_eq!(
            PYProject::generate(&project, py_config)
                .unwrap()
                .render()
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "libs/py/.gitignore".into(),
                        code: r#"__pycache__
dist"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/pyproject.toml".into(),
                        code: r#"[tool.poetry]
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
                    GTLangProjectSource {
                        path: "libs/py/module/py.typed".into(),
                        code: "".into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/__init__.py".into(),
                        code: r#"from .prompt import Prompt


__all__ = ["Prompt"]"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "libs/py/module/prompt.py".into(),
                        code: r#"from genotype_json import JsonAny
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
