use crossbeam::channel::{unbounded, Receiver};
use genotype_parser::{
    parser::parse_code,
    tree::module::{parse_module, Module},
};
use glob::glob;
use std::{
    collections::HashSet,
    fs::read_to_string,
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
    thread,
};

pub fn load_program(pattern: &str) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let processed_paths = Arc::new(Mutex::new(HashSet::new()));
    let modules = Arc::new(Mutex::new(vec![]));
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
    modules: Arc<Mutex<Vec<Module>>>,
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

        match load_module(&path) {
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
                modules.push(module);
            }
            Err(e) => {
                panic!("Error loading module: {:?}", e);
            }
        }
    }
}

fn load_module(path: &PathBuf) -> Result<(Module, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let code = read_to_string(&path)?;

    let pairs = parse_code(&code)?;
    let tree = parse_module(pairs)?;

    // [TODO]
    let deps = vec![];

    Ok((tree, deps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob() {
        let pattern = "./examples/basic/*.type";
        let result = load_program(pattern);
        match result {
            Ok(modules) => {
                assert_eq!(
                    modules,
                    vec![Module {
                        doc: None,
                        imports: vec![],
                        aliases: vec![],
                    }]
                );
            }

            Err(err) => {
                println!("{}", err);
                assert!(false, "Failed to load program");
            }
        }
    }
}
