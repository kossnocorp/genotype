use genotype_visitor::traverse::GTTraverse;
use glob::glob;
use rayon::Scope;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::{module::GTProjectModule, path::GTProjectPath, resolve::GTProjectResolveVisitor};

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

        let root = Arc::new(root);
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

        let mut modules = modules.lock().unwrap().clone();
        modules.sort_by(|a, b| a.path.as_path().cmp(&b.path.as_path()));

        let mut exports = HashMap::new();
        for module in modules.iter_mut() {
            exports.insert(module.path.clone(), module.exports.clone());
        }

        let mut resolver = GTProjectResolveVisitor::new(exports);
        for module in modules.iter_mut() {
            module.module.traverse(&mut resolver);
        }

        Ok(GTProject {
            root: (*root).clone(),
            modules,
        })
    }
}

fn process_module(
    root: Arc<GTProjectPath>,
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

    match GTProjectModule::load(&root, path.clone()) {
        Ok(module) => {
            for dep in module.deps.iter() {
                let root = Arc::clone(&root);
                let dep = dep.clone();
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    process_module(root, dep, scope, processed_paths, modules);
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
        object::GTObject, path::GTPath, primitive::GTPrimitive, property::GTProperty,
        reference::GTReference,
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
                    exports: vec!["Author".into()],
                    module: GTModule {
                        path: "author".into(),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Author".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: "name".into(),
                                    descriptor: GTPrimitive::String.into(),
                                    required: true,
                                }],
                            }),
                        }],
                    },
                },
                GTProjectModule {
                    path: "./examples/basic/book.type".try_into().unwrap(),
                    deps: vec!["./examples/basic/author.type".try_into().unwrap()],
                    exports: vec!["Book".into()],
                    module: GTModule {
                        path: "book".into(),
                        doc: None,
                        imports: vec![GTImport {
                            path: GTPath("./author".into()),
                            reference: GTImportReference::Name("Author".into()),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Book".into(),
                            descriptor: GTDescriptor::Object(GTObject {
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
                                        descriptor: GTDescriptor::Reference(GTReference::External(
                                            "Author".into(),
                                            "./author".into(),
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
                    exports: vec!["Order".into()],
                    module: GTModule {
                        path: "order".into(),
                        doc: None,
                        imports: vec![GTImport {
                            path: GTPath("./book".into()),
                            reference: GTImportReference::Name("Book".into()),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "Order".into(),
                            descriptor: GTDescriptor::Object(GTObject {
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: "user".into(),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            path: GTPath("./user".into()),
                                            name: "User".into(),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: "books".into(),
                                        descriptor: GTDescriptor::Array(Box::new(GTArray {
                                            descriptor: GTDescriptor::Reference(
                                                GTReference::External(
                                                    "Book".into(),
                                                    "./book".into(),
                                                ),
                                            ),
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
                    exports: vec!["User".into()],
                    module: GTModule {
                        path: "user".into(),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: "User".into(),
                            descriptor: GTDescriptor::Object(GTObject {
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
                },
            ],
        }
    }
}
