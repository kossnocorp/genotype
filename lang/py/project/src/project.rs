use std::{collections::HashSet, path::PathBuf};

use genotype_lang_py_config::PYProjectConfig;
use genotype_lang_py_tree::{py_indent, PYDefinition, PYRender};

use genotype_lang_core_project::{
    module::GTLangProjectModule,
    project::{GTLangProject, GTLangProjectRender},
    source::GTLangProjectSource,
};
use genotype_project::project::GTProject;

use crate::module::PYProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProject {
    pub modules: Vec<PYProjectModule>,
}

impl GTLangProject<PYProjectConfig> for PYProject {
    fn generate(
        project: &GTProject,
        config: &PYProjectConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let modules = project
            .modules
            .iter()
            .map(|module| PYProjectModule::generate(&project, module, config))
            .collect::<Result<_, _>>()?;

        Ok(Self { modules })
    }

    fn render(
        &self,
        config: &PYProjectConfig,
    ) -> Result<GTLangProjectRender, Box<dyn std::error::Error>> {
        let gitignore = GTLangProjectSource {
            path: config.package_path(".gitignore".into()),
            code: r#"__pycache__"#.into(),
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
            path: config.package_path("pyproject.toml".into()),
            code: format!(
                r#"[tool.poetry]{}
packages = [{{ include = "{}" }}]

[tool.poetry.dependencies]
{}{}

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#,
                if let Some(package) = &config.package {
                    format!("\n{}", package)
                } else {
                    "".into()
                },
                config.module,
                config.lang.version.as_dependency_str(),
                dependencies.iter().fold("".into(), |acc, dependency| {
                    if let Some(str) = dependency.external_str() {
                        format!("{acc}\n{str}")
                    } else {
                        acc
                    }
                })
            ),
        };

        let (imports, exports) = self
            .modules
            .iter()
            .fold((vec![], vec![]), |mut acc, module| {
                acc.0
                    .push(format!("from .{} import *", module.name.clone()));

                for definition in module.module.definitions.iter() {
                    acc.1.push(format!(
                        "\"{}\"",
                        match definition {
                            PYDefinition::Class(class) => class.name.0.clone(),
                            PYDefinition::Alias(alias) => alias.name.0.clone(),
                        }
                    ));
                }
                acc
            });
        let init = GTLangProjectSource {
            path: config.source_path("__init__.py".into()),
            code: format!(
                "{}\n\n\n__all__ = [{}]",
                imports.join("\n"),
                exports.join(", ")
            ),
        };

        let module_root = config.module_root_path();
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
                code: module.module.render(&py_indent(), &config.lang),
            })
            .collect::<Vec<_>>();

        let mut modules = vec![gitignore, pyproject, init];
        modules.extend(module_inits);
        modules.extend(project_modules);

        Ok(GTLangProjectRender { files: modules })
    }
}

#[cfg(test)]
mod tests {
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
            PYProject::generate(&project, &py_config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
                        path: "py/module/author.py".into(),
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
                                    name: "name".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                    required: true,
                                }],
                            })]
                        },
                    },
                    PYProjectModule {
                        name: "book".into(),
                        path: "py/module/book.py".into(),
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
                                        name: "title".into(),
                                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                        required: true,
                                    },
                                    PYProperty {
                                        name: "author".into(),
                                        descriptor: PYReference::new("Author".into(), false).into(),
                                        required: true,
                                    },
                                ],
                            })],
                        },
                    },
                ]
            },
        )
    }

    #[test]
    fn test_convert_glob() {
        let config = GTConfig::from_root("module", "./examples/glob");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &py_config).unwrap(),
            PYProject {
                modules: vec![
                    PYProjectModule {
                        name: "author".into(),
                        path: "py/module/author.py".into(),
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
                                PYDefinition::Class(PYClass {
                                    doc: None,
                                    name: "Author".into(),
                                    extensions: vec![],
                                    properties: vec![PYProperty {
                                        name: "name".into(),
                                        descriptor: PYReference::new("AuthorName".into(), true)
                                            .into(),
                                        required: true,
                                    }],
                                }),
                                PYDefinition::Alias(PYAlias {
                                    doc: None,
                                    name: "AuthorName".into(),
                                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                })
                            ]
                        },
                    },
                    PYProjectModule {
                        name: "book".into(),
                        path: "py/module/book.py".into(),
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
                                        name: "title".into(),
                                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                        required: true,
                                    },
                                    PYProperty {
                                        name: "author".into(),
                                        descriptor: PYReference::new("author.Author".into(), false)
                                            .into(),
                                        required: true,
                                    },
                                    PYProperty {
                                        name: "author_name".into(),
                                        descriptor: PYReference::new(
                                            "author.AuthorName".into(),
                                            false
                                        )
                                        .into(),
                                        required: true,
                                    },
                                ],
                            })],
                        },
                    },
                ]
            },
        )
    }

    #[test]
    fn test_render() {
        let config = GTConfig::from_root("module", "./examples/basic");
        let py_config = config.as_python_project().unwrap();
        let project = GTProject::load(&config).unwrap();

        assert_eq!(
            PYProject::generate(&project, &py_config)
                .unwrap()
                .render(&py_config)
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "py/.gitignore".into(),
                        code: "__pycache__".into(),
                    },
                    GTLangProjectSource {
                        path: "py/pyproject.toml".into(),
                        code: r#"[tool.poetry]
packages = [{ include = "module" }]

[tool.poetry.dependencies]
python = "^3.12"
genotype-runtime = "^0.3"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "py/module/__init__.py".into(),
                        code: r#"from .author import *
from .book import *


__all__ = ["Author", "Book"]"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "py/module/author.py".into(),
                        code: r#"from genotype import Model


class Author(Model):
    name: str
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "py/module/book.py".into(),
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
            PYProject::generate(&project, &py_config)
                .unwrap()
                .render(&py_config)
                .unwrap(),
            GTLangProjectRender {
                files: vec![
                    GTLangProjectSource {
                        path: "py/.gitignore".into(),
                        code: "__pycache__".into(),
                    },
                    GTLangProjectSource {
                        path: "py/pyproject.toml".into(),
                        code: r#"[tool.poetry]
packages = [{ include = "module" }]

[tool.poetry.dependencies]
python = "^3.12"
genotype-runtime = "^0.3"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                        .into(),
                    },
                    GTLangProjectSource {
                        path: "py/module/__init__.py".into(),
                        code: r#"from .inventory import *
from .shop.goods.book import *


__all__ = ["Inventory", "Book"]"#
                            .into(),
                    },
                    GTLangProjectSource {
                        path: "py/module/shop/goods/__init__.py".into(),
                        code: "".into(),
                    },
                    GTLangProjectSource {
                        path: "py/module/inventory.py".into(),
                        code: r#"from .shop.goods.book import Book
from genotype import Model


class Inventory(Model):
    goods: list[Book]
"#
                        .into()
                    },
                    GTLangProjectSource {
                        path: "py/module/shop/goods/book.py".into(),
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
}
