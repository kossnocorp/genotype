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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProject<'a> {
    pub modules: Vec<GtProjectModule>,
    pub config: &'a GtConfig,
}

impl<'a> GtProject<'a> {
    pub fn load(config: &'a GtConfig) -> Result<Self> {
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
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_glob() {
        let config = basic_config();
        let project = GtProject::load(&config);
        assert_ron_snapshot!(project.unwrap(), @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "author.type",
              module: GTModule(
                id: GTModuleId("author"),
                doc: None,
                imports: [],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("author"), "Author"),
                    span: GTSpan(0, 27),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(0, 6), "Author"),
                    descriptor: Object(GTObject(
                      span: GTSpan(8, 27),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(0, 6), "Author")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(12, 24),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(12, 16), "name"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(18, 24),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
              ),
              source_code: NamedSource(
                name: "author.type",
                source: "Author: {\n  name: string,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "book.type",
              module: GTModule(
                id: GTModuleId("book"),
                doc: None,
                imports: [
                  GTImport(
                    span: GTSpan(0, 19),
                    path: GTPath(GTSpan(4, 12), Resolved(GTModuleId("author")), "./author"),
                    reference: Name(GTSpan(13, 19), GTIdentifier(GTSpan(13, 19), "Author")),
                  ),
                ],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("book"), "Book"),
                    span: GTSpan(21, 65),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(21, 25), "Book"),
                    descriptor: Object(GTObject(
                      span: GTSpan(27, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(21, 25), "Book")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(31, 44),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(31, 36), "title"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(38, 44),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(48, 62),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(48, 54), "author"),
                          descriptor: Reference(GTReference(
                            span: GTSpan(56, 62),
                            doc: None,
                            attributes: [],
                            id: GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
                            definition_id: Resolved(GTDefinitionId(GTModuleId("author"), "Author")),
                            identifier: GTIdentifier(GTSpan(56, 62), "Author"),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {
                  GTPath(GTSpan(4, 12), Unresolved, "./author"): "author.type",
                },
                identifiers: {
                  GTIdentifier(GTSpan(56, 62), "Author"): GTPModuleIdentifierResolve(
                    source: External(GTPath(GTSpan(4, 12), Unresolved, "./author")),
                  ),
                },
                definitions: {
                  GTDefinitionId(GTModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                    references: [
                      GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
                    ],
                    deps: [],
                  ),
                },
              ),
              source_code: NamedSource(
                name: "book.type",
                source: "use ./author/Author\n\nBook: {\n  title: string,\n  author: Author,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "order.type",
              module: GTModule(
                id: GTModuleId("order"),
                doc: None,
                imports: [
                  GTImport(
                    span: GTSpan(0, 15),
                    path: GTPath(GTSpan(4, 10), Resolved(GTModuleId("book")), "./book"),
                    reference: Name(GTSpan(11, 15), GTIdentifier(GTSpan(11, 15), "Book")),
                  ),
                ],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("order"), "Order"),
                    span: GTSpan(17, 65),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(17, 22), "Order"),
                    descriptor: Object(GTObject(
                      span: GTSpan(24, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(17, 22), "Order")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(28, 45),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(28, 32), "user"),
                          descriptor: InlineImport(GTInlineImport(
                            span: GTSpan(34, 45),
                            name: GTIdentifier(GTSpan(41, 45), "User"),
                            path: GTPath(GTSpan(34, 40), Resolved(GTModuleId("user")), "./user"),
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(49, 62),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(49, 54), "books"),
                          descriptor: Array(GTArray(
                            span: GTSpan(56, 62),
                            descriptor: Reference(GTReference(
                              span: GTSpan(57, 61),
                              doc: None,
                              attributes: [],
                              id: GTReferenceId(GTModuleId("order"), GTSpan(57, 61)),
                              definition_id: Resolved(GTDefinitionId(GTModuleId("book"), "Book")),
                              identifier: GTIdentifier(GTSpan(57, 61), "Book"),
                            )),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {
                  GTPath(GTSpan(4, 10), Unresolved, "./book"): "book.type",
                  GTPath(GTSpan(34, 40), Unresolved, "./user"): "user.type",
                },
                identifiers: {
                  GTIdentifier(GTSpan(57, 61), "Book"): GTPModuleIdentifierResolve(
                    source: External(GTPath(GTSpan(4, 10), Unresolved, "./book")),
                  ),
                },
                definitions: {
                  GTDefinitionId(GTModuleId("book"), "Book"): GtProjectModuleDefinitionResolve(
                    references: [
                      GTReferenceId(GTModuleId("order"), GTSpan(57, 61)),
                    ],
                    deps: [],
                  ),
                },
              ),
              source_code: NamedSource(
                name: "order.type",
                source: "use ./book/Book\n\nOrder: {\n  user: ./user/User,\n  books: [Book],\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "user.type",
              module: GTModule(
                id: GTModuleId("user"),
                doc: None,
                imports: [],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("user"), "User"),
                    span: GTSpan(0, 42),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(0, 4), "User"),
                    descriptor: Object(GTObject(
                      span: GTSpan(6, 42),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(0, 4), "User")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(10, 23),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(10, 15), "email"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(27, 39),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(27, 31), "name"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(33, 39),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
              ),
              source_code: NamedSource(
                name: "user.type",
                source: "User: {\n  email: string,\n  name: string,\n}",
                language: None,
              ),
            ),
          ],
          config: GtConfig(
            name: Some("module"),
            root: "examples/basic",
            out: "dist",
            src: "",
            entry: "**/*.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "enabled": false,
              "out": "py",
              "manifest": {},
              "dependencies": {},
            },
            rs: {
              "derive": [
                "Debug",
                "Clone",
                "PartialEq",
              ],
              "enabled": false,
              "out": "rs",
              "manifest": {},
              "dependencies": {},
            },
          ),
        )
        "#);
    }

    #[test]
    fn test_entry() {
        let config = GtConfig::from_entry("module", "./examples/basic", "order.type");
        let project = GtProject::load(&config);
        assert_ron_snapshot!(project.unwrap(), @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "author.type",
              module: GTModule(
                id: GTModuleId("author"),
                doc: None,
                imports: [],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("author"), "Author"),
                    span: GTSpan(0, 27),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(0, 6), "Author"),
                    descriptor: Object(GTObject(
                      span: GTSpan(8, 27),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(0, 6), "Author")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(12, 24),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(12, 16), "name"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(18, 24),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
              ),
              source_code: NamedSource(
                name: "author.type",
                source: "Author: {\n  name: string,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "book.type",
              module: GTModule(
                id: GTModuleId("book"),
                doc: None,
                imports: [
                  GTImport(
                    span: GTSpan(0, 19),
                    path: GTPath(GTSpan(4, 12), Resolved(GTModuleId("author")), "./author"),
                    reference: Name(GTSpan(13, 19), GTIdentifier(GTSpan(13, 19), "Author")),
                  ),
                ],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("book"), "Book"),
                    span: GTSpan(21, 65),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(21, 25), "Book"),
                    descriptor: Object(GTObject(
                      span: GTSpan(27, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(21, 25), "Book")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(31, 44),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(31, 36), "title"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(38, 44),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(48, 62),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(48, 54), "author"),
                          descriptor: Reference(GTReference(
                            span: GTSpan(56, 62),
                            doc: None,
                            attributes: [],
                            id: GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
                            definition_id: Resolved(GTDefinitionId(GTModuleId("author"), "Author")),
                            identifier: GTIdentifier(GTSpan(56, 62), "Author"),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {
                  GTPath(GTSpan(4, 12), Unresolved, "./author"): "author.type",
                },
                identifiers: {
                  GTIdentifier(GTSpan(56, 62), "Author"): GTPModuleIdentifierResolve(
                    source: External(GTPath(GTSpan(4, 12), Unresolved, "./author")),
                  ),
                },
                definitions: {
                  GTDefinitionId(GTModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                    references: [
                      GTReferenceId(GTModuleId("book"), GTSpan(56, 62)),
                    ],
                    deps: [],
                  ),
                },
              ),
              source_code: NamedSource(
                name: "book.type",
                source: "use ./author/Author\n\nBook: {\n  title: string,\n  author: Author,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "order.type",
              module: GTModule(
                id: GTModuleId("order"),
                doc: None,
                imports: [
                  GTImport(
                    span: GTSpan(0, 15),
                    path: GTPath(GTSpan(4, 10), Resolved(GTModuleId("book")), "./book"),
                    reference: Name(GTSpan(11, 15), GTIdentifier(GTSpan(11, 15), "Book")),
                  ),
                ],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("order"), "Order"),
                    span: GTSpan(17, 65),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(17, 22), "Order"),
                    descriptor: Object(GTObject(
                      span: GTSpan(24, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(17, 22), "Order")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(28, 45),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(28, 32), "user"),
                          descriptor: InlineImport(GTInlineImport(
                            span: GTSpan(34, 45),
                            name: GTIdentifier(GTSpan(41, 45), "User"),
                            path: GTPath(GTSpan(34, 40), Resolved(GTModuleId("user")), "./user"),
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(49, 62),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(49, 54), "books"),
                          descriptor: Array(GTArray(
                            span: GTSpan(56, 62),
                            descriptor: Reference(GTReference(
                              span: GTSpan(57, 61),
                              doc: None,
                              attributes: [],
                              id: GTReferenceId(GTModuleId("order"), GTSpan(57, 61)),
                              definition_id: Resolved(GTDefinitionId(GTModuleId("book"), "Book")),
                              identifier: GTIdentifier(GTSpan(57, 61), "Book"),
                            )),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {
                  GTPath(GTSpan(4, 10), Unresolved, "./book"): "book.type",
                  GTPath(GTSpan(34, 40), Unresolved, "./user"): "user.type",
                },
                identifiers: {
                  GTIdentifier(GTSpan(57, 61), "Book"): GTPModuleIdentifierResolve(
                    source: External(GTPath(GTSpan(4, 10), Unresolved, "./book")),
                  ),
                },
                definitions: {
                  GTDefinitionId(GTModuleId("book"), "Book"): GtProjectModuleDefinitionResolve(
                    references: [
                      GTReferenceId(GTModuleId("order"), GTSpan(57, 61)),
                    ],
                    deps: [],
                  ),
                },
              ),
              source_code: NamedSource(
                name: "order.type",
                source: "use ./book/Book\n\nOrder: {\n  user: ./user/User,\n  books: [Book],\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "user.type",
              module: GTModule(
                id: GTModuleId("user"),
                doc: None,
                imports: [],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("user"), "User"),
                    span: GTSpan(0, 42),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(0, 4), "User"),
                    descriptor: Object(GTObject(
                      span: GTSpan(6, 42),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(0, 4), "User")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(10, 23),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(10, 15), "email"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GTProperty(
                          span: GTSpan(27, 39),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(27, 31), "name"),
                          descriptor: Primitive(GTPrimitive(
                            span: GTSpan(33, 39),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
              ),
              source_code: NamedSource(
                name: "user.type",
                source: "User: {\n  email: string,\n  name: string,\n}",
                language: None,
              ),
            ),
          ],
          config: GtConfig(
            name: Some("module"),
            root: "examples/basic",
            out: "dist",
            src: "",
            entry: "order.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "enabled": false,
              "out": "py",
              "manifest": {},
              "dependencies": {},
            },
            rs: {
              "derive": [
                "Debug",
                "Clone",
                "PartialEq",
              ],
              "enabled": false,
              "out": "rs",
              "manifest": {},
              "dependencies": {},
            },
          ),
        )
        "#);
    }

    #[test]
    fn test_process_anonymous() {
        let module_path: GtModulePath = "anonymous.type".into();
        let config = GtConfig::from_entry("module", "./examples/process", "anonymous.type");
        let project = GtProject::load(&config);
        assert_ron_snapshot!(
            project.unwrap(),
            @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "anonymous.type",
              module: GTModule(
                id: GTModuleId("anonymous"),
                doc: None,
                imports: [],
                aliases: [
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("anonymous"), "Order"),
                    span: GTSpan(0, 91),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(0, 5), "Order"),
                    descriptor: Object(GTObject(
                      span: GTSpan(7, 91),
                      doc: None,
                      attributes: [],
                      name: Named(GTIdentifier(GTSpan(0, 5), "Order")),
                      extensions: [],
                      properties: [
                        GTProperty(
                          span: GTSpan(11, 89),
                          doc: None,
                          attributes: [],
                          name: GTKey(GTSpan(11, 19), "delivery"),
                          descriptor: Object(GTObject(
                            span: GTSpan(21, 89),
                            doc: None,
                            attributes: [],
                            name: Alias(GTIdentifier(GTSpan(21, 89), "OrderDelivery"), Property(GTIdentifier(GTSpan(0, 5), "Order"), [
                              GTKey(GTSpan(11, 19), "delivery"),
                            ])),
                            extensions: [],
                            properties: [
                              GTProperty(
                                span: GTSpan(27, 85),
                                doc: None,
                                attributes: [],
                                name: GTKey(GTSpan(27, 34), "address"),
                                descriptor: Object(GTObject(
                                  span: GTSpan(36, 85),
                                  doc: None,
                                  attributes: [],
                                  name: Alias(GTIdentifier(GTSpan(36, 85), "OrderDeliveryAddress"), Property(GTIdentifier(GTSpan(0, 5), "Order"), [
                                    GTKey(GTSpan(11, 19), "delivery"),
                                    GTKey(GTSpan(27, 34), "address"),
                                  ])),
                                  extensions: [],
                                  properties: [
                                    GTProperty(
                                      span: GTSpan(44, 58),
                                      doc: None,
                                      attributes: [],
                                      name: GTKey(GTSpan(44, 50), "street"),
                                      descriptor: Primitive(GTPrimitive(
                                        span: GTSpan(52, 58),
                                        kind: String,
                                        doc: None,
                                        attributes: [],
                                      )),
                                      required: true,
                                    ),
                                    GTProperty(
                                      span: GTSpan(66, 78),
                                      doc: None,
                                      attributes: [],
                                      name: GTKey(GTSpan(66, 70), "city"),
                                      descriptor: Primitive(GTPrimitive(
                                        span: GTSpan(72, 78),
                                        kind: String,
                                        doc: None,
                                        attributes: [],
                                      )),
                                      required: true,
                                    ),
                                  ],
                                )),
                                required: true,
                              ),
                            ],
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                  GTAlias(
                    id: GTDefinitionId(GTModuleId("anonymous"), "Email"),
                    span: GTSpan(93, 145),
                    doc: None,
                    attributes: [],
                    name: GTIdentifier(GTSpan(93, 98), "Email"),
                    descriptor: Union(GTUnion(
                      span: GTSpan(100, 145),
                      descriptors: [
                        Primitive(GTPrimitive(
                          span: GTSpan(100, 106),
                          kind: String,
                          doc: None,
                          attributes: [],
                        )),
                        Object(GTObject(
                          span: GTSpan(109, 145),
                          doc: None,
                          attributes: [],
                          name: Alias(GTIdentifier(GTSpan(109, 145), "EmailObj"), Alias(GTIdentifier(GTSpan(93, 98), "Email"))),
                          extensions: [],
                          properties: [
                            GTProperty(
                              span: GTSpan(113, 125),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(113, 117), "name"),
                              descriptor: Primitive(GTPrimitive(
                                span: GTSpan(119, 125),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
                              required: true,
                            ),
                            GTProperty(
                              span: GTSpan(129, 142),
                              doc: None,
                              attributes: [],
                              name: GTKey(GTSpan(129, 134), "email"),
                              descriptor: Primitive(GTPrimitive(
                                span: GTSpan(136, 142),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
                              required: true,
                            ),
                          ],
                        )),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GTPModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
              ),
              source_code: NamedSource(
                name: "anonymous.type",
                source: "Order: {\n  delivery: {\n    address: {\n      street: string,\n      city: string,\n    }\n  }\n}\n\nEmail: string | {\n  name: string,\n  email: string,\n}\n\n",
                language: None,
              ),
            ),
          ],
          config: GtConfig(
            name: Some("module"),
            root: "examples/process",
            out: "dist",
            src: "",
            entry: "anonymous.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "enabled": false,
              "out": "py",
              "manifest": {},
              "dependencies": {},
            },
            rs: {
              "derive": [
                "Debug",
                "Clone",
                "PartialEq",
              ],
              "enabled": false,
              "out": "rs",
              "manifest": {},
              "dependencies": {},
            },
          ),
        )
        "#
        );
    }

    fn basic_config() -> GtConfig {
        GtConfig::from_root("module", "./examples/basic")
    }
}
