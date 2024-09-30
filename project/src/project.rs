use genotype_visitor::traverse::GTTraverse;
use glob::glob;
use rayon::Scope;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{module::GTProjectModule, visitor::GTProjectVisitor};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProject {
    pub modules: HashSet<GTProjectModule>,
}

impl GTProject {
    pub fn load(pattern: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let processed_paths = Arc::new(Mutex::new(HashSet::new()));
        let modules = Arc::new(Mutex::new(HashSet::new()));

        let result = glob(pattern)?;

        let entries: Vec<PathBuf> = result
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|p| p.canonicalize())
            .collect::<Result<Vec<_>, _>>()?;

        rayon::scope(|scope| {
            for entry in entries {
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(entry, scope, processed_paths, modules);
                });
            }
        });

        let modules = modules.lock().unwrap().clone();
        Ok(GTProject { modules })
    }
}

fn process_module(
    path: PathBuf,
    scope: &Scope<'_>,
    processed_paths: Arc<Mutex<HashSet<PathBuf>>>,
    modules: Arc<Mutex<HashSet<GTProjectModule>>>,
) {
    {
        let mut processed = processed_paths.lock().unwrap();
        if processed.contains(&path) {
            return;
        } else {
            processed.insert(path.clone());
        }
    }

    match load_module(path) {
        Ok((module, deps)) => {
            for dep in deps {
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(dep, scope, processed_paths, modules);
                });
            }

            let mut modules = modules.lock().unwrap();
            modules.insert(module);
        }

        Err(err) => {
            panic!("Error loading module: {:?}", err);
        }
    }
}

fn load_module(
    path: PathBuf,
) -> Result<(GTProjectModule, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let module = TryInto::<GTProjectModule>::try_into(path)?;

    let mut visitor = GTProjectVisitor { deps: vec![] };
    module.traverse(&mut visitor);

    let dir = module.path.parent().unwrap();

    let deps = visitor
        .deps
        .into_iter()
        .map(|p| {
            let path = dir.join(p + ".type");
            path.canonicalize()
        })
        .collect::<Result<Vec<PathBuf>, std::io::Error>>()?;

    println!("********* Module: {:?}", module);
    println!("********* Deps: {:?}", deps);

    Ok((module, deps))
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
        let project = GTProject::load("./examples/basic/*.type");
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
        let project = GTProject::load("./examples/basic/order.type");
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
            modules: vec![
                GTProjectModule {
                    path: canonical_path("./examples/basic/author.type"),
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Author".to_string()),
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
                    path: canonical_path("./examples/basic/book.type"),
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./author".to_string(),
                            reference: GTImportReference::Name(GTName("Author".into())),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Book".to_string()),
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
                    path: canonical_path("./examples/basic/order.type"),
                    module: GTModule {
                        doc: None,
                        imports: vec![GTImport {
                            path: "./book".to_string(),
                            reference: GTImportReference::Name(GTName("Book".into())),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("Order".to_string()),
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
                    path: canonical_path("./examples/basic/user.type"),
                    module: GTModule {
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTName("User".to_string()),
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
            ]
            .into_iter()
            .collect(),
        }
    }

    fn canonical_path(path: &str) -> PathBuf {
        PathBuf::from(path).canonicalize().unwrap()
    }
}
