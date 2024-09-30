use genotype_parser::{
    parser::parse_gt_code,
    tree::module::{parse_gt_module, GTModule},
};
use genotype_visitor::traverse::GTTraverse;
use glob::glob;
use rayon::Scope;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::visitor::GTProjectVisitor;

pub fn load_gt_project(
    pattern: &str,
) -> Result<HashMap<PathBuf, GTModule>, Box<dyn std::error::Error>> {
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
    modules: Arc<Mutex<HashMap<PathBuf, GTModule>>>,
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

fn load_module(
    module_path: PathBuf,
) -> Result<(GTModule, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let code = read_to_string(&module_path)?;

    let pairs = parse_gt_code(&code)?;
    let module = parse_gt_module(module_path.to_str().unwrap().to_string(), pairs)?;

    let mut visitor = GTProjectVisitor { deps: vec![] };
    module.traverse(&mut visitor);

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
        alias::GTAlias,
        array::GTArray,
        descriptor::GTDescriptor,
        import::{GTImport, ImportReference},
        name::GTName,
        object::GTObject,
        primitive::GTPrimitive,
        property::GTProperty,
        reference::GTReference,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_glob() {
        let result = load_gt_project("./examples/basic/*.type");
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
        let result = load_gt_project("./examples/basic/order.type");
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

    fn basic_project() -> Vec<GTModule> {
        vec![
            GTModule {
                path: module_path("./examples/basic/author.type"),
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
            GTModule {
                path: module_path("./examples/basic/book.type"),
                doc: None,
                imports: vec![GTImport {
                    path: "./author".to_string(),
                    reference: ImportReference::Name("Author".to_string()),
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
                                descriptor: GTDescriptor::Name(GTName("Author".to_string())),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
            GTModule {
                path: module_path("./examples/basic/order.type"),
                doc: None,
                imports: vec![GTImport {
                    path: "./book".to_string(),
                    reference: ImportReference::Name("Book".to_string()),
                }],
                aliases: vec![GTAlias {
                    doc: None,
                    name: GTName("Order".to_string()),
                    descriptor: GTDescriptor::Object(GTObject {
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: GTName("user".to_string()),
                                descriptor: GTDescriptor::Reference(GTReference {
                                    path: "./user".to_string(),
                                    name: GTName("User".to_string()),
                                }),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: GTName("books".to_string()),
                                descriptor: GTDescriptor::Array(Box::new(GTArray {
                                    descriptor: GTDescriptor::Name(GTName("Book".to_string())),
                                })),
                                required: true,
                            },
                        ],
                    }),
                }],
            },
            GTModule {
                path: module_path("./examples/basic/user.type"),
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
        ]
    }

    fn order_modules(modules: HashMap<PathBuf, GTModule>) -> Vec<GTModule> {
        let mut modules = modules
            .iter()
            .map(|(_, module)| module.clone())
            .collect::<Vec<GTModule>>();
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
