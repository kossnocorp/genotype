use crossbeam::channel::{unbounded, Receiver};
use genotype_parser::{
    parser::parse_code,
    tree::module::{parse_module, Module},
};
use glob::glob;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
    thread,
};

pub fn load_project(pattern: &str) -> Result<HashMap<PathBuf, Module>, Box<dyn std::error::Error>> {
    let processed_paths = Arc::new(Mutex::new(HashSet::new()));
    let modules = Arc::new(Mutex::new(HashMap::new()));
    let (sender, receiver) = unbounded();

    let result = glob(pattern)?;

    for path in result {
        let path = path?.canonicalize()?;
        sender.send(path).unwrap();
    }

    let sender = Arc::new(sender);
    let weak_sender = Arc::downgrade(&sender);

    let num_workers = num_cpus::get();
    let mut handles = Vec::new();

    for _ in 0..num_workers {
        let receiver = receiver.clone();
        let weak_sender = weak_sender.clone();
        let processed_paths = Arc::clone(&processed_paths);
        let modules = Arc::clone(&modules);

        let handle =
            thread::spawn(move || worker_thread(receiver, weak_sender, processed_paths, modules));
        handles.push(handle);
    }

    drop(sender);

    for handle in handles {
        handle.join().unwrap();
    }

    let modules = modules.lock().unwrap().clone();
    Ok(modules)
}

fn worker_thread(
    receiver: Receiver<PathBuf>,
    weak_sender: Weak<crossbeam::channel::Sender<PathBuf>>,
    processed_paths: Arc<Mutex<HashSet<PathBuf>>>,
    modules: Arc<Mutex<HashMap<PathBuf, Module>>>,
) {
    while let Ok(path) = receiver.recv() {
        {
            let mut processed = processed_paths.lock().unwrap();
            if processed.contains(&path) {
                continue;
            } else {
                processed.insert(path.clone());
            }
        }

        match load_module(path) {
            Ok((module, deps)) => {
                for dep_path in deps {
                    match dep_path.canonicalize() {
                        Ok(dep_path) => {
                            if let Some(sender) = weak_sender.upgrade() {
                                sender.send(dep_path).unwrap();
                            } else {
                                break;
                            }
                        }
                        Err(e) => {
                            panic!("Error canonicalizing path: {:?}", e);
                        }
                    }
                }

                let mut modules = modules.lock().unwrap();
                modules.insert(module.path.clone(), module);
            }
            Err(e) => {
                panic!("Error loading module: {:?}", e);
            }
        }
    }
}

fn load_module(path: PathBuf) -> Result<(Module, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let code = read_to_string(&path)?;

    let pairs = parse_code(&code)?;
    let tree = parse_module(path, pairs)?;

    // [TODO]
    let deps = vec![];

    Ok((tree, deps))
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
        let pattern = "./examples/basic/*.type";
        let result = load_project(pattern);
        match result {
            Ok(modules) => {
                assert_eq!(
                    modules,
                    HashMap::from([
                        module_kv(Module {
                            path: PathBuf::from("./examples/basic/author.type")
                                .canonicalize()
                                .unwrap(),
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
                                        required: true
                                    }]
                                })
                            }]
                        }),
                        module_kv(Module {
                            path: PathBuf::from("./examples/basic/book.type")
                                .canonicalize()
                                .unwrap(),
                            doc: None,
                            imports: vec![Import {
                                path: "./author/".to_string(),
                                reference: ImportReference::Name("Author".to_string())
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
                                            required: true
                                        },
                                        Property {
                                            doc: None,
                                            name: "author".to_string(),
                                            descriptor: Descriptor::Name("Author".to_string()),
                                            required: true
                                        }
                                    ]
                                })
                            }]
                        }),
                        module_kv(Module {
                            path: PathBuf::from("./examples/basic/order.type")
                                .canonicalize()
                                .unwrap(),
                            doc: None,
                            imports: vec![Import {
                                path: "./book/".to_string(),
                                reference: ImportReference::Name("Book".to_string())
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
                                                path: "./users".to_string(),
                                                name: "User".to_string()
                                            }),
                                            required: true
                                        },
                                        Property {
                                            doc: None,
                                            name: "books".to_string(),
                                            descriptor: Descriptor::Array(Box::new(Array {
                                                descriptor: Descriptor::Name("Book".to_string())
                                            })),
                                            required: true
                                        }
                                    ]
                                })
                            }]
                        }),
                        module_kv(Module {
                            path: PathBuf::from("./examples/basic/user.type")
                                .canonicalize()
                                .unwrap(),
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
                                            required: true
                                        },
                                        Property {
                                            doc: None,
                                            name: "name".to_string(),
                                            descriptor: Descriptor::Primitive(Primitive::String),
                                            required: true
                                        }
                                    ]
                                })
                            }]
                        })
                    ])
                );
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load project");
            }
        }
    }

    fn module_kv(module: Module) -> (PathBuf, Module) {
        (module.path.clone(), module)
    }
}
