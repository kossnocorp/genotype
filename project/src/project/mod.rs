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
    use std::{collections::HashMap, fs::read_to_string};

    use crate::{GTProjectModuleReference, GTProjectModuleResolve};

    use super::*;
    use genotype_parser::{tree::*, GTSourceCode};
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
                        source_code: GTSourceCode::new(
                            author_path.as_name(),
                            read_to_string(&author_path).unwrap(),
                        ),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 6).into(), "Author".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    doc: None,
                                    name: GTKey::new((13, 17).into(), "name".into()),
                                    descriptor: GTPrimitive::String((19, 25).into()).into(),
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
                        source_code: GTSourceCode::new(
                            book_path.as_name(),
                            read_to_string(&book_path).unwrap(),
                        ),
                        doc: None,
                        imports: vec![GTImport {
                            span: (0, 19).into(),
                            path: GTPath::parse((4, 12).into(), "./author").unwrap(),
                            reference: GTImportReference::Name(
                                (13, 19).into(),
                                GTIdentifier::new((13, 19).into(), "Author".into()),
                            ),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTIdentifier::new((21, 25).into(), "Book".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((32, 37).into(), "title".into()),
                                        descriptor: GTPrimitive::String((39, 45).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((48, 54).into(), "author".into()),
                                        descriptor: GTDescriptor::Reference(
                                            GTIdentifier::new((56, 62).into(), "Author".into())
                                                .into(),
                                        ),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::from_iter([(
                            GTPath::parse((4, 12).into(), "./author").unwrap(),
                            Arc::new(author_path.clone()),
                        )]),
                        references: HashMap::from_iter([(
                            GTIdentifier::new((56, 62).into(), "Author".into()),
                            GTProjectModuleReference::External(
                                GTPath::parse((4, 12).into(), "./author").unwrap(),
                            ),
                        )]),
                    },
                },
                GTProjectModule {
                    path: order_path.clone(),
                    module: GTModule {
                        source_code: GTSourceCode::new(
                            order_path.as_name(),
                            read_to_string(&order_path).unwrap(),
                        ),
                        doc: None,
                        imports: vec![GTImport {
                            span: (0, 15).into(),
                            path: GTPath::parse((4, 10).into(), "./book").unwrap(),
                            reference: GTImportReference::Name(
                                (11, 15).into(),
                                GTIdentifier::new((11, 15).into(), "Book".into()),
                            ),
                        }],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTIdentifier::new((17, 22).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((29, 33).into(), "user".into()),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            span: (35, 46).into(),
                                            path: GTPath::parse((35, 41).into(), "./user").unwrap(),
                                            name: GTIdentifier::new((42, 46).into(), "User".into()),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((49, 54).into(), "books".into()),
                                        descriptor: GTDescriptor::Array(Box::new(GTArray {
                                            span: (56, 62).into(),
                                            descriptor: GTDescriptor::Reference(
                                                GTIdentifier::new((57, 61).into(), "Book".into())
                                                    .into(),
                                            ),
                                        })),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTProjectModuleResolve {
                        deps: HashMap::from_iter([
                            (
                                GTPath::parse((4, 10).into(), "./book").unwrap(),
                                Arc::new(book_path.clone()),
                            ),
                            (
                                GTPath::parse((35, 41).into(), "./user").unwrap(),
                                Arc::new(user_path.clone()),
                            ),
                        ]),
                        references: HashMap::from_iter([(
                            GTIdentifier::new((57, 61).into(), "Book".into()),
                            GTProjectModuleReference::External(
                                GTPath::parse((4, 10).into(), "./book").unwrap(),
                            ),
                        )]),
                    },
                },
                GTProjectModule {
                    path: user_path.clone(),
                    module: GTModule {
                        source_code: GTSourceCode::new(
                            user_path.as_name(),
                            read_to_string(&user_path).unwrap(),
                        ),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            doc: None,
                            name: GTIdentifier::new((0, 4).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((11, 16).into(), "email".into()),
                                        descriptor: GTPrimitive::String((18, 24).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        doc: None,
                                        name: GTKey::new((27, 31).into(), "name".into()),
                                        descriptor: GTPrimitive::String((33, 39).into()).into(),
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
