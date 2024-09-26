use genotype_parser::{
    parser::parse_code,
    tree::module::{parse_module, Module},
};
use genotype_visitor::traverse::traverse_module;
use glob::glob;
use rayon::Scope;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::visitor::ProjectVisitor;

pub fn load_project(pattern: &str) -> Result<HashMap<PathBuf, Module>, Box<dyn std::error::Error>> {
    let processed_paths = Arc::new(Mutex::new(HashSet::new()));
    let modules = Arc::new(Mutex::new(HashMap::new()));

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
    Ok(modules)
}

fn process_module(
    path: PathBuf,
    scope: &Scope<'_>,
    processed_paths: Arc<Mutex<HashSet<PathBuf>>>,
    modules: Arc<Mutex<HashMap<PathBuf, Module>>>,
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
            let path = PathBuf::from(module.path.clone()).canonicalize().unwrap();
            modules.insert(path, module);
        }

        Err(e) => {
            panic!("Error loading module: {:?}", e);
        }
    }
}

fn load_module(module_path: PathBuf) -> Result<(Module, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let code = read_to_string(&module_path)?;

    let pairs = parse_code(&code)?;
    let module = parse_module(module_path.to_str().unwrap().to_string(), pairs)?;

    let mut visitor = ProjectVisitor { deps: vec![] };
    traverse_module(&module, &mut visitor);

    let dir = module_path.parent().unwrap();

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
        alias::Alias,
        array::Array,
        descriptor::Descriptor,
        import::{Import, ImportReference},
        object::Object,
        primitive::Primitive,
        property::Property,
        reference::Reference,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_glob() {
        let result = load_project("./examples/basic/*.type");
        match result {
            Ok(modules) => {
                assert_eq!(order_modules(modules), basic_project());
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load project");
            }
        }
    }

    #[test]
    fn test_entry() {
        let result = load_project("./examples/basic/order.type");
        match result {
            Ok(modules) => {
                assert_eq!(order_modules(modules), basic_project());
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load project");
            }
        }
    }

    fn basic_project() -> Vec<Module> {
        vec![
            Module {
                path: module_path("./examples/basic/author.type"),
                doc: None,
                imports: vec![],
                aliases: vec![Alias {
                    doc: None,
                    name: "Author".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![Property {
                            doc: None,
                            name: "name".to_string(),
                            descriptor: Descriptor::Primitive(Primitive::String),
                            required: true,
                        }],
                    }),
                }],
            },
            Module {
                path: module_path("./examples/basic/book.type"),
                doc: None,
                imports: vec![Import {
                    path: "./author".to_string(),
                    reference: ImportReference::Name("Author".to_string()),
                }],
                aliases: vec![Alias {
                    doc: None,
                    name: "Book".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![
                            Property {
                                doc: None,
                                name: "title".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            },
                            Property {
                                doc: None,
                                name: "author".to_string(),
                                descriptor: Descriptor::Name("Author".to_string()),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
            Module {
                path: module_path("./examples/basic/order.type"),
                doc: None,
                imports: vec![Import {
                    path: "./book".to_string(),
                    reference: ImportReference::Name("Book".to_string()),
                }],
                aliases: vec![Alias {
                    doc: None,
                    name: "Order".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![
                            Property {
                                doc: None,
                                name: "user".to_string(),
                                descriptor: Descriptor::Reference(Reference {
                                    path: "./user".to_string(),
                                    name: "User".to_string(),
                                }),
                                required: true,
                            },
                            Property {
                                doc: None,
                                name: "books".to_string(),
                                descriptor: Descriptor::Array(Box::new(Array {
                                    descriptor: Descriptor::Name("Book".to_string()),
                                })),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
            Module {
                path: module_path("./examples/basic/user.type"),
                doc: None,
                imports: vec![],
                aliases: vec![Alias {
                    doc: None,
                    name: "User".to_string(),
                    descriptor: Descriptor::Object(Object {
                        properties: vec![
                            Property {
                                doc: None,
                                name: "email".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            },
                            Property {
                                doc: None,
                                name: "name".to_string(),
                                descriptor: Descriptor::Primitive(Primitive::String),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
        ]
    }

    fn order_modules(modules: HashMap<PathBuf, Module>) -> Vec<Module> {
        let mut modules = modules
            .iter()
            .map(|(_, module)| module.clone())
            .collect::<Vec<Module>>();
        modules.sort_by(|a, b| a.path.cmp(&b.path));
        modules
    }

    fn module_path(path: &str) -> String {
        PathBuf::from(path)
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
