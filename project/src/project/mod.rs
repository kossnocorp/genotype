use glob::glob;
use rayon::Scope;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{GTProjectModule, GTProjectModuleParse, GTProjectModulePath};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProject {
    pub root: Arc<PathBuf>,
    pub modules: Vec<GTProjectModule>,
}

impl GTProject {
    pub fn load(root: &str, pattern: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root = Arc::new(PathBuf::from(root).canonicalize()?);

        let entry_paths = glob(root.join(pattern).to_str().unwrap())?;
        let entries: Vec<GTProjectModulePath> = entry_paths
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|entry| GTProjectModulePath::try_new(Arc::clone(&root), entry))
            .collect::<Result<Vec<_>, _>>()?;

        let processed_paths = Arc::new(Mutex::new(HashSet::new()));
        let modules = Arc::new(Mutex::new(Vec::new()));

        rayon::scope(|scope| {
            for entry in entries {
                let root = Arc::clone(&root);
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(root, entry, scope, processed_paths, modules);
                });
            }
        });

        let modules = modules.lock().unwrap().clone();

        let mut modules = modules
            .iter()
            .map(|parse| GTProjectModule::try_new(&modules, parse.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        modules.sort_by(|a, b| a.path.as_path().cmp(&b.path.as_path()));

        Ok(GTProject {
            root: root.clone(),
            modules,
        })
    }
}

fn process_module(
    root: Arc<PathBuf>,
    path: GTProjectModulePath,
    scope: &Scope<'_>,
    processed_paths: Arc<Mutex<HashSet<GTProjectModulePath>>>,
    modules: Arc<Mutex<Vec<GTProjectModuleParse>>>,
) {
    {
        let mut processed = processed_paths.lock().unwrap();
        if processed.contains(&path) {
            return;
        } else {
            processed.insert(path.clone());
        }
    }

    let parse = GTProjectModuleParse::try_new(path).unwrap();

    for path in parse.deps().unwrap() {
        let root = Arc::clone(&root);
        let processed_paths = Arc::clone(&processed_paths);
        let modules = Arc::clone(&modules);

        scope.spawn(|scope| {
            process_module(root, path, scope, processed_paths, modules);
        });
    }

    let mut modules = modules.lock().unwrap();
    modules.push(parse);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{GTProjectModuleReference, GTProjectModuleResolve};

    use super::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_glob() {
        let project = GTProject::load("./examples/basic", "*.type");
        match project {
            Ok(project) => {
                assert_eq!(project, basic_project());
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load project");
            }
        }
    }

    #[test]
    fn test_entry() {
        let project = GTProject::load("./examples/basic", "order.type");
        match project {
            Ok(project) => {
                assert_eq!(project, basic_project());
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load project");
            }
        }
    }

    fn basic_project() -> GTProject {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let author_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/author.type"),
        )
        .unwrap();
        let book_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/book.type"),
        )
        .unwrap();
        let order_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/order.type"),
        )
        .unwrap();
        let user_path = GTProjectModulePath::try_new(
            root.clone(),
            &PathBuf::from("./examples/basic/user.type"),
        )
        .unwrap();

        GTProject {
            root: root.clone(),
            modules: vec![
                GTProjectModule {
                    path: author_path.clone(),
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Author".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTPrimitive::String.into(),
                                    required: true,
                                }],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::new(),
                        references: HashMap::new(),
                    },
                },
                GTProjectModule {
                    path: book_path.clone(),
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./author".into(),
                            reference: GTImportReference::Name("Author".into()),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Book".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "title".into(),
                                        descriptor: GTPrimitive::String.into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "author".into(),
                                        descriptor: GTDescriptor::Reference("Author".into()),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::from_iter([(
                            "./author".into(),
                            Arc::new(author_path.clone()),
                        )]),
                        references: HashMap::from_iter([(
                            "Author".into(),
                            GTProjectModuleReference::External("./author".into()),
                        )]),
                    },
                },
                GTProjectModule {
                    path: order_path.clone(),
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./book".into(),
                            reference: GTImportReference::Name("Book".into()),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Order".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "user".into(),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            path: "./user".into(),
                                            name: "User".into(),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "books".into(),
                                        descriptor: GTDescriptor::Array(Box::new(GTArray {
                                            descriptor: GTDescriptor::Reference("Book".into()),
                                        })),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::from_iter([
                            ("./book".into(), Arc::new(book_path.clone())),
                            ("./user".into(), Arc::new(user_path.clone())),
                        ]),
                        references: HashMap::from_iter([(
                            "Book".into(),
                            GTProjectModuleReference::External("./book".into()),
                        )]),
                    },
                },
                GTProjectModule {
                    path: user_path.clone(),
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "User".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "email".into(),
                                        descriptor: GTPrimitive::String.into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "name".into(),
                                        descriptor: GTPrimitive::String.into(),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::new(),
                        references: HashMap::new(),
                    },
                },
            ],
        }
    }
}
