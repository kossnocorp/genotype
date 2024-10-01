use glob::glob;
use rayon::Scope;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::{module::GTProjectModule, path::GTProjectPath};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProject {
    pub root: GTProjectPath,
    pub modules: Vec<GTProjectModule>,
}

impl GTProject {
    pub fn load(root: &str, pattern: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root: GTProjectPath = root.try_into()?;

        let entry_paths = glob(root.as_path().join(pattern).to_str().unwrap())?;
        let entries: Vec<GTProjectPath> = entry_paths
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(GTProjectPath::try_new)
            .collect::<Result<Vec<_>, _>>()?;

        let processed_paths = Arc::new(Mutex::new(HashSet::new()));
        let modules = Arc::new(Mutex::new(Vec::new()));

        rayon::scope(|scope| {
            for entry in entries {
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(entry, scope, processed_paths, modules);
                });
            }
        });

        let mut modules = modules.lock().unwrap().clone();
        modules.sort_by(|a, b| a.path.as_path().cmp(&b.path.as_path()));

        Ok(GTProject { root, modules })
    }
}

fn process_module(
    path: GTProjectPath,
    scope: &Scope<'_>,
    processed_paths: Arc<Mutex<HashSet<GTProjectPath>>>,
    modules: Arc<Mutex<Vec<GTProjectModule>>>,
) {
    {
        let mut processed = processed_paths.lock().unwrap();
        if processed.contains(&path) {
            return;
        } else {
            processed.insert(path.clone());
        }
    }

    match GTProjectModule::load(path.clone()) {
        Ok(module) => {
            for dep in module.deps.iter() {
                let dep = dep.clone();
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(dep, scope, processed_paths, modules);
                });
            }

            let mut modules = modules.lock().unwrap();
            modules.push(module);
        }

        Err(err) => {
            panic!("======== Error loading module {:?}: ${:?}", path, err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::tree::{
        alias::GTAlias, array::GTArray, descriptor::GTDescriptor, import::GTImport,
        import_reference::GTImportReference, inline_import::GTInlineImport, module::GTModule,
        name::GTName, object::GTObject, primitive::GTPrimitive, property::GTProperty,
    };
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
        GTProject {
            root: "./examples/basic".try_into().unwrap(),
            modules: vec![
                GTProjectModule {
                    path: "./examples/basic/author.type".try_into().unwrap(),
                    deps: vec![],
                    exports: vec![GTName("Author".into())],
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Author".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: GTName("name".to_string()),
                                    descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                    required: true,
                                }],
                            }),
                        }],
                    },
                },
                GTProjectModule {
                    path: "./examples/basic/book.type".try_into().unwrap(),
                    deps: vec!["./examples/basic/author.type".try_into().unwrap()],
                    exports: vec![GTName("Book".into())],
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./author".to_string(),
                            reference: GTImportReference::Name(GTName("Author".into())),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Book".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTName("title".to_string()),
                                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTName("author".to_string()),
                                        descriptor: GTDescriptor::Name(GTName(
                                            "Author".to_string(),
                                        )),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                },
                GTProjectModule {
                    path: "./examples/basic/order.type".try_into().unwrap(),
                    deps: vec![
                        "./examples/basic/book.type".try_into().unwrap(),
                        "./examples/basic/user.type".try_into().unwrap(),
                    ],
                    exports: vec![GTName("Order".into())],
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./book".to_string(),
                            reference: GTImportReference::Name(GTName("Book".into())),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTName("user".to_string()),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            path: "./user".to_string(),
                                            name: GTName("User".to_string()),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTName("books".to_string()),
                                        descriptor: GTDescriptor::Array(Box::new(GTArray {
                                            descriptor: GTDescriptor::Name(GTName(
                                                "Book".to_string(),
                                            )),
                                        })),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                },
                GTProjectModule {
                    path: "./examples/basic/user.type".try_into().unwrap(),
                    deps: vec![],
                    exports: vec![GTName("User".into())],
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTName("email".to_string()),
                                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTName("name".to_string()),
                                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                },
            ],
        }
    }
}
