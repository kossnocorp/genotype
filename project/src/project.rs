use crate::prelude::internal::*;
use genotype_path::GtRelativePath;
use genotype_path::*;
use glob::glob;
use miette::Result;
use rayon::Scope;
use relative_path::RelativePathBuf;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq, Clone)]
pub struct GtProject {
    pub modules: Vec<GtProjectModule>,
    pub config: GtConfig,
}

impl GtProject {
    pub fn load(config: GtConfig) -> Result<Self> {
        let src_path = config.src_path();
        let entries = glob(config.entry_path().as_str())
            .map_err(|_| GTProjectError::Unknown)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GTProjectError::Unknown)?
            .into_iter()
            .map(|path| {
                RelativePathBuf::from_path(path)
                    .map_err(|_| GTProjectError::Unknown)
                    .and_then(|path| {
                        path.strip_prefix(src_path.relative_path().normalize())
                            .map_err(|_| GTProjectError::Unknown)
                            .and_then(|path| Ok(GtModulePath::new(path.into())))
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GTProjectError::Unknown)?;

        if entries.is_empty() {
            return Err(GTProjectError::NoEntries(config.entry_path().as_str().into()).into());
        }

        let processed_paths = Arc::new(Mutex::new(HashSet::new()));
        let modules: Arc<Mutex<Vec<Result<GTProjectModuleParse>>>> =
            Arc::new(Mutex::new(Vec::new()));

        rayon::scope(|scope| {
            let config = Arc::new(config.clone());

            for entry in entries {
                let config = Arc::clone(&config);
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    Self::load_module(config, entry, scope, processed_paths, modules)
                });
            }
        });

        // [TODO] Simplify and turn into errors
        let modules_parse = Arc::try_unwrap(modules)
            .expect("Mutex cannot be unwrapped")
            .into_inner()
            .expect("Mutex cannot be locked")
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        let resolve: GTPResolve = (&modules_parse).try_into()?;

        let mut modules = modules_parse
            .iter()
            .map(|parse| GtProjectModule::try_new(&resolve, &modules_parse, parse.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        // [TODO] It's needed for tests, hide behind cfg(test), keep or replace with something like
        // set? Using HashSet will require Eq which will consequently break tests.
        modules.sort_by(|a, b| a.path.as_str().cmp(&b.path.as_str()));

        Ok(GtProject { modules, config })
    }

    fn load_module(
        config: Arc<GtConfig>,
        module_path: GtModulePath,
        scope: &Scope<'_>,
        processed_paths: Arc<Mutex<HashSet<GtModulePath>>>,
        modules: Arc<Mutex<Vec<Result<GTProjectModuleParse>>>>,
    ) {
        // Check if the module is already processed to avoid infinite recursion.
        {
            let mut processed = processed_paths.lock().expect("Failed to lock modules");
            if processed.contains(&module_path) {
                return;
            }
            processed.insert(module_path.clone());
        }

        let result = GTProjectModuleParse::try_new(&config, module_path).and_then(|parse| {
            parse.deps().and_then(|deps| {
                // Iterate each module dependency and load it in a thread.
                for dep in deps {
                    let config = Arc::clone(&config);
                    let processed_paths = Arc::clone(&processed_paths);
                    let modules = Arc::clone(&modules);

                    scope.spawn(|scope| {
                        Self::load_module(config, dep, scope, processed_paths, modules);
                    });
                }

                // Push the module parse result to the modules vector.
                {
                    let mut modules = modules.lock().expect("Failed to lock modules");
                    modules.push(Ok(parse));
                }

                Ok(())
            })
        });

        // If parsing failed, push the error to the modules vector.
        if let Err(err) = result {
            let mut modules = modules.lock().expect("Failed to lock modules");
            modules.push(Err(err));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::read_to_string};

    use crate::{GTPModuleIdentifierSource, GTPModuleResolve};

    use super::*;
    use genotype_parser::*;
    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_glob() {
        let project = GtProject::load(GtConfig::from_root("module", "./examples/basic"));
        assert_eq!(project.unwrap(), basic_project());
    }

    #[test]
    fn test_entry() {
        let config = GtConfig::from_entry("module", "./examples/basic", "order.type");
        let project = GtProject::load(config);
        assert_eq!(project.unwrap(), basic_project());
    }

    #[test]
    fn test_process_anonymous() {
        let module_path: GtModulePath = "/process/anonymous.type".into();
        let config = GtConfig::from_entry("module", "./examples/process", "anonymous.type");
        let project = GtProject::load(config.clone());
        assert_eq!(
            project.unwrap(),
            GtProject {
                modules: vec![GtProjectModule {
                    path: module_path.clone(),
                    module: GTModule {
                        id: "anonymous".into(),
                        doc: None,
                        imports: vec![],
                        aliases: vec![
                            GTAlias {
                                id: GTDefinitionId("anonymous".into(), "Order".into()),
                                span: (0, 91).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((0, 5).into(), "Order".into()),
                                descriptor: GTObject {
                                    span: (8, 91).into(),
                                    name: GTIdentifier::new((0, 5).into(), "Order".into()).into(),
                                    extensions: vec![],
                                    properties: vec![GTProperty {
                                        span: (12, 89).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((12, 20).into(), "delivery".into()),
                                        descriptor: GTObject {
                                            span: (22, 89).into(),
                                            name: GTObjectName::Alias(
                                                GTIdentifier::new(
                                                    (22, 89).into(),
                                                    "OrderDelivery".into()
                                                ),
                                                GTObjectNameParent::Property(
                                                    GTIdentifier::new(
                                                        (0, 5).into(),
                                                        "Order".into()
                                                    ),
                                                    vec![GTKey::new(
                                                        (12, 20).into(),
                                                        "delivery".into()
                                                    )]
                                                ),
                                            ),
                                            extensions: vec![],
                                            properties: vec![GTProperty {
                                                span: (28, 85).into(),
                                                doc: None,
                                                attributes: vec![],
                                                name: GTKey::new((28, 35).into(), "address".into()),
                                                descriptor: GTObject {
                                                    span: (37, 85).into(),
                                                    name: GTObjectName::Alias(
                                                        GTIdentifier::new(
                                                            (37, 85).into(),
                                                            "OrderDeliveryAddress".into()
                                                        ),
                                                        GTObjectNameParent::Property(
                                                            GTIdentifier::new(
                                                                (0, 5).into(),
                                                                "Order".into()
                                                            ),
                                                            vec![
                                                                GTKey::new(
                                                                    (12, 20).into(),
                                                                    "delivery".into()
                                                                ),
                                                                GTKey::new(
                                                                    (28, 35).into(),
                                                                    "address".into()
                                                                )
                                                            ]
                                                        ),
                                                    ),
                                                    extensions: vec![],
                                                    properties: vec![
                                                        GTProperty {
                                                            span: (45, 59).into(),
                                                            doc: None,
                                                            attributes: vec![],
                                                            name: GTKey::new(
                                                                (45, 51).into(),
                                                                "street".into()
                                                            ),
                                                            descriptor: GTPrimitive::String(
                                                                (53, 59).into()
                                                            )
                                                            .into(),
                                                            required: true,
                                                        },
                                                        GTProperty {
                                                            span: (67, 79).into(),
                                                            doc: None,
                                                            attributes: vec![],
                                                            name: GTKey::new(
                                                                (67, 71).into(),
                                                                "city".into()
                                                            ),
                                                            descriptor: GTPrimitive::String(
                                                                (73, 79).into()
                                                            )
                                                            .into(),
                                                            required: true,
                                                        }
                                                    ],
                                                }
                                                .into(),
                                                required: true,
                                            }],
                                        }
                                        .into(),
                                        required: true,
                                    }],
                                }
                                .into(),
                            },
                            GTAlias {
                                id: GTDefinitionId("anonymous".into(), "Email".into()),
                                span: (93, 146).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTIdentifier::new((93, 98).into(), "Email".into()),
                                descriptor: GTUnion {
                                    span: (101, 146).into(),
                                    descriptors: vec![
                                        GTPrimitive::String((101, 107).into()).into(),
                                        GTObject {
                                            span: (110, 146).into(),
                                            name: GTObjectName::Alias(
                                                GTIdentifier::new(
                                                    (110, 146).into(),
                                                    "EmailObj".into()
                                                ),
                                                GTObjectNameParent::Alias(GTIdentifier::new(
                                                    (93, 98).into(),
                                                    "Email".into()
                                                ),),
                                            ),
                                            extensions: vec![],
                                            properties: vec![
                                                GTProperty {
                                                    span: (114, 126).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (114, 118).into(),
                                                        "name".into()
                                                    ),
                                                    descriptor: GTPrimitive::String(
                                                        (120, 126).into()
                                                    )
                                                    .into(),
                                                    required: true,
                                                },
                                                GTProperty {
                                                    span: (130, 143).into(),
                                                    doc: None,
                                                    attributes: vec![],
                                                    name: GTKey::new(
                                                        (130, 135).into(),
                                                        "email".into()
                                                    ),
                                                    descriptor: GTPrimitive::String(
                                                        (137, 143).into()
                                                    )
                                                    .into(),
                                                    required: true,
                                                }
                                            ],
                                        }
                                        .into(),
                                    ]
                                }
                                .into()
                            }
                        ],
                    },
                    resolve: GTPModuleResolve {
                        paths: Default::default(),
                        identifiers: Default::default(),
                        definitions: Default::default(),
                    },
                    source_code: NamedSource::new(
                        "anonymous.type",
                        read_to_string(
                            config.src_path().join(&module_path.clone().into()).as_str()
                        )
                        .unwrap(),
                    ),
                }],
                config
            }
        );
    }

    fn basic_project() -> GtProject {
        let config = GtConfig::from_root("module", "./examples/basic");

        let author_path: GtModulePath = "author.type".into();
        let book_path: GtModulePath = "book.type".into();
        let order_path: GtModulePath = "order.type".into();
        let user_path: GtModulePath = "user.type".into();

        GtProject {
            modules: vec![
                GtProjectModule {
                    path: author_path.clone(),
                    module: GTModule {
                        id: "author".into(),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            id: GTDefinitionId("author".into(), "Author".into()),
                            span: (0, 27).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 6).into(), "Author".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (9, 27).into(),
                                name: GTIdentifier::new((0, 6).into(), "Author".into()).into(),
                                extensions: vec![],
                                properties: vec![GTProperty {
                                    span: (13, 25).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GTKey::new((13, 17).into(), "name".into()),
                                    descriptor: GTPrimitive::String((19, 25).into()).into(),
                                    required: true,
                                }],
                            }),
                        }],
                    },
                    resolve: GTPModuleResolve {
                        paths: Default::default(),
                        identifiers: Default::default(),
                        definitions: HashMap::from_iter([]),
                    },
                    source_code: NamedSource::new(
                        "author.type",
                        read_to_string(
                            config.src_path().join(&author_path.clone().into()).as_str(),
                        )
                        .unwrap(),
                    ),
                },
                GtProjectModule {
                    path: book_path.clone(),
                    module: GTModule {
                        id: "book".into(),
                        doc: None,
                        imports: vec![GTImport {
                            span: (0, 19).into(),
                            path: GTPath::new(
                                (4, 12).into(),
                                GTPathModuleId::Resolved(GTModuleId("author".into())),
                                "./author".into(),
                            ),
                            reference: GTImportReference::Name(
                                (13, 19).into(),
                                GTIdentifier::new((13, 19).into(), "Author".into()),
                            ),
                        }],
                        aliases: vec![GTAlias {
                            id: GTDefinitionId("book".into(), "Book".into()),
                            span: (21, 64).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((21, 25).into(), "Book".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (28, 64).into(),
                                name: GTIdentifier::new((21, 25).into(), "Book".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (32, 45).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((32, 37).into(), "title".into()),
                                        descriptor: GTPrimitive::String((39, 45).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (48, 62).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((48, 54).into(), "author".into()),
                                        descriptor: GTDescriptor::Reference(GTReference {
                                            span: (56, 62).into(),
                                            id: GTReferenceId("book".into(), (56, 62).into()),
                                            definition_id: GTReferenceDefinitionId::Resolved(
                                                GTDefinitionId("author".into(), "Author".into()),
                                            ),
                                            identifier: GTIdentifier::new(
                                                (56, 62).into(),
                                                "Author".into(),
                                            ),
                                        }),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTPModuleResolve {
                        paths: HashMap::from_iter([(
                            GTPath::parse((4, 12).into(), "./author").unwrap(),
                            author_path,
                        )]),
                        identifiers: HashMap::from_iter([(
                            GTIdentifier::new((56, 62).into(), "Author".into()),
                            GTPModuleIdentifierResolve {
                                source: GTPModuleIdentifierSource::External(
                                    GTPath::parse((4, 12).into(), "./author").unwrap(),
                                ),
                            },
                        )]),
                        definitions: HashMap::from_iter([(
                            GTDefinitionId("author".into(), "Author".into()),
                            GTPModuleDefinitionResolve {
                                references: HashSet::from_iter([GTReferenceId(
                                    "book".into(),
                                    (56, 62).into(),
                                )]),
                                deps: Default::default(),
                            },
                        )]),
                    },
                    source_code: NamedSource::new(
                        "book.type",
                        read_to_string(config.src_path().join(&book_path.clone().into()).as_str())
                            .unwrap(),
                    ),
                },
                GtProjectModule {
                    path: order_path.clone(),
                    module: GTModule {
                        id: "order".into(),
                        doc: None,
                        imports: vec![GTImport {
                            span: (0, 15).into(),
                            path: GTPath::new(
                                (4, 10).into(),
                                GTPathModuleId::Resolved("book".into()),
                                "./book".into(),
                            ),
                            reference: GTImportReference::Name(
                                (11, 15).into(),
                                GTIdentifier::new((11, 15).into(), "Book".into()),
                            ),
                        }],
                        aliases: vec![GTAlias {
                            id: GTDefinitionId("order".into(), "Order".into()),
                            span: (17, 64).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((17, 22).into(), "Order".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (25, 64).into(),
                                name: GTIdentifier::new((17, 22).into(), "Order".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (29, 46).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((29, 33).into(), "user".into()),
                                        descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                            span: (35, 46).into(),
                                            path: GTPath::parse((35, 41).into(), "./user").unwrap(),
                                            name: GTIdentifier::new((42, 46).into(), "User".into()),
                                        }),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (49, 62).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((49, 54).into(), "books".into()),
                                        descriptor: GTDescriptor::Array(Box::new(GTArray {
                                            span: (56, 62).into(),
                                            descriptor: GTDescriptor::Reference(GTReference {
                                                span: (57, 61).into(),
                                                id: GTReferenceId("order".into(), (57, 61).into()),
                                                definition_id: GTReferenceDefinitionId::Resolved(
                                                    GTDefinitionId("book".into(), "Book".into()),
                                                ),
                                                identifier: GTIdentifier::new(
                                                    (57, 61).into(),
                                                    "Book".into(),
                                                ),
                                            }),
                                        })),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTPModuleResolve {
                        paths: HashMap::from_iter([
                            (GTPath::parse((4, 10).into(), "./book").unwrap(), book_path),
                            (
                                GTPath::parse((35, 41).into(), "./user").unwrap(),
                                user_path.clone(),
                            ),
                        ]),
                        identifiers: HashMap::from_iter([(
                            GTIdentifier::new((57, 61).into(), "Book".into()),
                            GTPModuleIdentifierResolve {
                                source: GTPModuleIdentifierSource::External(
                                    GTPath::parse((4, 10).into(), "./book").unwrap(),
                                ),
                            },
                        )]),
                        definitions: HashMap::from_iter([(
                            GTDefinitionId("book".into(), "Book".into()),
                            GTPModuleDefinitionResolve {
                                references: HashSet::from_iter([GTReferenceId(
                                    "order".into(),
                                    (57, 61).into(),
                                )]),
                                deps: Default::default(),
                            },
                        )]),
                    },
                    source_code: NamedSource::new(
                        "order.type",
                        read_to_string(config.src_path().join(&order_path.into()).as_str())
                            .unwrap(),
                    ),
                },
                GtProjectModule {
                    path: user_path.clone(),
                    module: GTModule {
                        id: "user".into(),
                        doc: None,
                        imports: vec![],
                        aliases: vec![GTAlias {
                            id: GTDefinitionId("user".into(), "User".into()),
                            span: (0, 41).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTIdentifier::new((0, 4).into(), "User".into()),
                            descriptor: GTDescriptor::Object(GTObject {
                                span: (7, 41).into(),
                                name: GTIdentifier::new((0, 4).into(), "User".into()).into(),
                                extensions: vec![],
                                properties: vec![
                                    GTProperty {
                                        span: (11, 24).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((11, 16).into(), "email".into()),
                                        descriptor: GTPrimitive::String((18, 24).into()).into(),
                                        required: true,
                                    },
                                    GTProperty {
                                        span: (27, 39).into(),
                                        doc: None,
                                        attributes: vec![],
                                        name: GTKey::new((27, 31).into(), "name".into()),
                                        descriptor: GTPrimitive::String((33, 39).into()).into(),
                                        required: true,
                                    },
                                ],
                            }),
                        }],
                    },
                    resolve: GTPModuleResolve {
                        paths: Default::default(),
                        identifiers: Default::default(),
                        definitions: Default::default(),
                    },
                    source_code: NamedSource::new(
                        "user.type",
                        read_to_string(config.src_path().join(&user_path.into()).as_str()).unwrap(),
                    ),
                },
            ],
            config,
        }
    }
}
