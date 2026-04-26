use crate::prelude::internal::*;
use genotype_project_core::GtpRelativePath;
use genotype_project_core::*;
use glob::glob;
use miette::{Context, Result};
use rayon::Scope;
use relative_path::RelativePathBuf;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

mod paths;
pub use paths::*;

mod pkg;

/// Genotype project. Represents configuration with currently loaded modules.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProject {
    /// Known project modules mapped by their workspace path.
    modules: HashMap<GtpModulePath, GtpModuleState>,
    /// Parsed project modules. Represents final state produced by legacy loading logic.
    #[deprecated]
    pub modules_legacy: Vec<GtpModule>,
    /// Project configuration.
    pub config: GtpConfig,
    /// Project paths.
    pub paths: GtpPaths,
}

impl GtProject {
    pub fn try_new(config_file_path: GtpConfigFilePath, config: GtpConfig) -> Result<Self> {
        let paths = GtpPaths::try_new(config_file_path, &config)
            .wrap_err("failed to initialize project paths from config")?;

        Ok(Self {
            modules: HashMap::new(),
            modules_legacy: Vec::new(),
            config,
            paths,
        })
    }

    pub fn init_module(&mut self, path: &GtpModulePath) -> bool {
        match self.has_module(path) {
            true => false,
            false => {
                self.modules.insert(path.clone(), GtpModuleState::Loading);
                true
            }
        }
    }

    pub fn has_module(&self, path: &GtpModulePath) -> bool {
        self.modules.contains_key(path)
    }

    pub fn set_module(&mut self, path: &GtpModulePath, module_state: GtpModuleState) {
        self.modules.insert(path.clone(), module_state);
    }

    pub fn load(config_file_path: GtpConfigFilePath, config: GtpConfig) -> Result<Self> {
        let paths = GtpPaths::try_new(config_file_path, &config)
            .wrap_err("failed to initialize project paths from config")?;

        // let entries = glob(config.entry_path().as_str())
        let entries = glob(".")
            .map_err(|_| GtpError::Unknown)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GtpError::Unknown)?
            .into_iter()
            .map(|path| {
                RelativePathBuf::from_path(path)
                    .map_err(|_| GtpError::Unknown)
                    .and_then(|path| {
                        path.strip_prefix(paths.src.relative_path().normalize())
                            .map_err(|_| GtpError::Unknown)
                            .map(|path| GtpSrcDirRelativeModulePath::new(path.into()))
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GtpError::Unknown)?;

        if entries.is_empty() {
            // return Err(GtpError::NoEntries(config.entry_path().as_str().into()).into());
            return Err(GtpError::NoEntries(".".into()).into());
        }

        let processed_paths = Arc::new(Mutex::new(HashSet::new()));
        let modules: Arc<Mutex<Vec<Result<GtpModuleParse>>>> = Arc::new(Mutex::new(Vec::new()));

        rayon::scope(|scope| {
            let config = Arc::new(config.clone());

            for entry in entries {
                let src_dir_path = Arc::new(paths.src.clone());
                let config = Arc::clone(&config);
                let processed_paths = Arc::clone(&processed_paths);
                let modules = Arc::clone(&modules);

                scope.spawn(|scope| {
                    Self::load_module(src_dir_path, config, entry, scope, processed_paths, modules)
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

        let resolve: GtpResolve = (&modules_parse).try_into()?;

        let mut modules = modules_parse
            .iter()
            .map(|parse| GtpModule::try_new(&resolve, &modules_parse, parse.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        // [TODO] It's needed for tests, hide behind cfg(test), keep or replace with something like
        // set? Using HashSet will require Eq which will consequently break tests.
        modules.sort_by(|a, b| a.path.as_str().cmp(b.path.as_str()));

        Ok(GtProject {
            modules: HashMap::new(),
            modules_legacy: modules,
            config,
            paths,
        })
    }

    fn load_module(
        src_dir_path: Arc<GtpSrcDirPath>,
        config: Arc<GtpConfig>,
        module_path: GtpSrcDirRelativeModulePath,
        scope: &Scope<'_>,
        processed_paths: Arc<Mutex<HashSet<GtpSrcDirRelativeModulePath>>>,
        modules: Arc<Mutex<Vec<Result<GtpModuleParse>>>>,
    ) {
        // Check if the module is already processed to avoid infinite recursion.
        {
            let mut processed = processed_paths.lock().expect("Failed to lock modules");
            if processed.contains(&module_path) {
                return;
            }
            processed.insert(module_path.clone());
        }

        let result =
            GtpModuleParse::try_new(&src_dir_path, &config, module_path).and_then(|parse| {
                parse.deps().map(|deps| {
                    // Iterate each module dependency and load it in a thread.
                    for dep in deps {
                        let src_dir_path = Arc::clone(&src_dir_path);
                        let config = Arc::clone(&config);
                        let processed_paths = Arc::clone(&processed_paths);
                        let modules = Arc::clone(&modules);

                        scope.spawn(|scope| {
                            Self::load_module(
                                src_dir_path,
                                config,
                                dep,
                                scope,
                                processed_paths,
                                modules,
                            );
                        });
                    }

                    // Push the module parse result to the modules vector.
                    {
                        let mut modules = modules.lock().expect("Failed to lock modules");
                        modules.push(Ok(parse));
                    }
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
    use insta::{assert_debug_snapshot, assert_ron_snapshot};

    #[test]
    fn test_glob() {
        let config = basic_config();
        let project = GtProject::load("genotype.toml".into(), config);
        assert_ron_snapshot!(project.unwrap(), @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "author.type",
              module: GtModule(
                id: GtModuleId("author"),
                doc: None,
                imports: [],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("author"), "Author"),
                    span: GtSpan(0, 27),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(0, 6), "Author"),
                    descriptor: Object(GtObject(
                      span: GtSpan(8, 27),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(0, 6), "Author")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(12, 24),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(12, 16), "name"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(18, 24),
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
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
              source_code: NamedSource(
                name: "author.type",
                source: "Author: {\n  name: string,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "book.type",
              module: GtModule(
                id: GtModuleId("book"),
                doc: None,
                imports: [
                  GtImport(
                    span: GtSpan(0, 19),
                    path: GtPath(
                      span: GtSpan(4, 12),
                      id: GtPathModuleId(
                        span: GtSpan(4, 12),
                        module_id: GtModuleId("book"),
                      ),
                      path: "./author",
                    ),
                    reference: Name(GtSpan(13, 19), GtIdentifier(GtSpan(13, 19), "Author")),
                  ),
                ],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("book"), "Book"),
                    span: GtSpan(21, 65),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(21, 25), "Book"),
                    descriptor: Object(GtObject(
                      span: GtSpan(27, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(21, 25), "Book")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(31, 44),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(31, 36), "title"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(38, 44),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(48, 62),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(48, 54), "author"),
                          descriptor: Reference(GtReference(
                            span: GtSpan(56, 62),
                            doc: None,
                            attributes: [],
                            id: GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                            identifier: GtIdentifier(GtSpan(56, 62), "Author"),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GtpModuleResolve(
                paths: {
                  GtPath(
                    span: GtSpan(4, 12),
                    id: GtPathModuleId(
                      span: GtSpan(4, 12),
                      module_id: GtModuleId("book"),
                    ),
                    path: "./author",
                  ): "author.type",
                },
                identifiers: {
                  GtIdentifier(GtSpan(56, 62), "Author"): GtpModuleIdentifierResolve(
                    source: External(GtPath(
                      span: GtSpan(4, 12),
                      id: GtPathModuleId(
                        span: GtSpan(4, 12),
                        module_id: GtModuleId("book"),
                      ),
                      path: "./author",
                    )),
                  ),
                },
                definitions: {
                  GtDefinitionId(GtModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                    references: [
                      GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                    ],
                    deps: [],
                  ),
                },
                reference_definition_ids: {
                  GtReferenceId(GtModuleId("book"), GtSpan(56, 62)): GtDefinitionId(GtModuleId("author"), "Author"),
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
              module: GtModule(
                id: GtModuleId("order"),
                doc: None,
                imports: [
                  GtImport(
                    span: GtSpan(0, 15),
                    path: GtPath(
                      span: GtSpan(4, 10),
                      id: GtPathModuleId(
                        span: GtSpan(4, 10),
                        module_id: GtModuleId("order"),
                      ),
                      path: "./book",
                    ),
                    reference: Name(GtSpan(11, 15), GtIdentifier(GtSpan(11, 15), "Book")),
                  ),
                ],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("order"), "Order"),
                    span: GtSpan(17, 65),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(17, 22), "Order"),
                    descriptor: Object(GtObject(
                      span: GtSpan(24, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(17, 22), "Order")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(28, 45),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(28, 32), "user"),
                          descriptor: InlineImport(GtInlineImport(
                            span: GtSpan(34, 45),
                            doc: None,
                            attributes: [],
                            name: GtIdentifier(GtSpan(41, 45), "User"),
                            path: GtPath(
                              span: GtSpan(34, 40),
                              id: GtPathModuleId(
                                span: GtSpan(34, 40),
                                module_id: GtModuleId("order"),
                              ),
                              path: "./user",
                            ),
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(49, 62),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(49, 54), "books"),
                          descriptor: Array(GtArray(
                            span: GtSpan(56, 62),
                            doc: None,
                            attributes: [],
                            descriptor: Reference(GtReference(
                              span: GtSpan(57, 61),
                              doc: None,
                              attributes: [],
                              id: GtReferenceId(GtModuleId("order"), GtSpan(57, 61)),
                              identifier: GtIdentifier(GtSpan(57, 61), "Book"),
                            )),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GtpModuleResolve(
                paths: {
                  GtPath(
                    span: GtSpan(4, 10),
                    id: GtPathModuleId(
                      span: GtSpan(4, 10),
                      module_id: GtModuleId("order"),
                    ),
                    path: "./book",
                  ): "book.type",
                  GtPath(
                    span: GtSpan(34, 40),
                    id: GtPathModuleId(
                      span: GtSpan(34, 40),
                      module_id: GtModuleId("order"),
                    ),
                    path: "./user",
                  ): "user.type",
                },
                identifiers: {
                  GtIdentifier(GtSpan(57, 61), "Book"): GtpModuleIdentifierResolve(
                    source: External(GtPath(
                      span: GtSpan(4, 10),
                      id: GtPathModuleId(
                        span: GtSpan(4, 10),
                        module_id: GtModuleId("order"),
                      ),
                      path: "./book",
                    )),
                  ),
                },
                definitions: {
                  GtDefinitionId(GtModuleId("book"), "Book"): GtProjectModuleDefinitionResolve(
                    references: [
                      GtReferenceId(GtModuleId("order"), GtSpan(57, 61)),
                    ],
                    deps: [],
                  ),
                },
                reference_definition_ids: {
                  GtReferenceId(GtModuleId("order"), GtSpan(57, 61)): GtDefinitionId(GtModuleId("book"), "Book"),
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
              module: GtModule(
                id: GtModuleId("user"),
                doc: None,
                imports: [],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("user"), "User"),
                    span: GtSpan(0, 42),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(0, 4), "User"),
                    descriptor: Object(GtObject(
                      span: GtSpan(6, 42),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(0, 4), "User")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(10, 23),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(10, 15), "email"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(27, 39),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(27, 31), "name"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(33, 39),
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
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
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
            version: None,
            root: "examples/basic",
            out: "dist",
            src: "",
            entry: "**/*.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "mode": types,
              "prefer": interface,
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "manager": poetry,
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
        let config = GtpConfig::from_entry("module", "./examples/basic", "order.type");
        let project = GtProject::load("genotype.toml".into(), config);
        assert_ron_snapshot!(project.unwrap(), @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "author.type",
              module: GtModule(
                id: GtModuleId("author"),
                doc: None,
                imports: [],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("author"), "Author"),
                    span: GtSpan(0, 27),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(0, 6), "Author"),
                    descriptor: Object(GtObject(
                      span: GtSpan(8, 27),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(0, 6), "Author")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(12, 24),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(12, 16), "name"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(18, 24),
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
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
              source_code: NamedSource(
                name: "author.type",
                source: "Author: {\n  name: string,\n}",
                language: None,
              ),
            ),
            GtProjectModule(
              path: "book.type",
              module: GtModule(
                id: GtModuleId("book"),
                doc: None,
                imports: [
                  GtImport(
                    span: GtSpan(0, 19),
                    path: GtPath(
                      span: GtSpan(4, 12),
                      id: GtPathModuleId(
                        span: GtSpan(4, 12),
                        module_id: GtModuleId("book"),
                      ),
                      path: "./author",
                    ),
                    reference: Name(GtSpan(13, 19), GtIdentifier(GtSpan(13, 19), "Author")),
                  ),
                ],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("book"), "Book"),
                    span: GtSpan(21, 65),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(21, 25), "Book"),
                    descriptor: Object(GtObject(
                      span: GtSpan(27, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(21, 25), "Book")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(31, 44),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(31, 36), "title"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(38, 44),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(48, 62),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(48, 54), "author"),
                          descriptor: Reference(GtReference(
                            span: GtSpan(56, 62),
                            doc: None,
                            attributes: [],
                            id: GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                            identifier: GtIdentifier(GtSpan(56, 62), "Author"),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GtpModuleResolve(
                paths: {
                  GtPath(
                    span: GtSpan(4, 12),
                    id: GtPathModuleId(
                      span: GtSpan(4, 12),
                      module_id: GtModuleId("book"),
                    ),
                    path: "./author",
                  ): "author.type",
                },
                identifiers: {
                  GtIdentifier(GtSpan(56, 62), "Author"): GtpModuleIdentifierResolve(
                    source: External(GtPath(
                      span: GtSpan(4, 12),
                      id: GtPathModuleId(
                        span: GtSpan(4, 12),
                        module_id: GtModuleId("book"),
                      ),
                      path: "./author",
                    )),
                  ),
                },
                definitions: {
                  GtDefinitionId(GtModuleId("author"), "Author"): GtProjectModuleDefinitionResolve(
                    references: [
                      GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                    ],
                    deps: [],
                  ),
                },
                reference_definition_ids: {
                  GtReferenceId(GtModuleId("book"), GtSpan(56, 62)): GtDefinitionId(GtModuleId("author"), "Author"),
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
              module: GtModule(
                id: GtModuleId("order"),
                doc: None,
                imports: [
                  GtImport(
                    span: GtSpan(0, 15),
                    path: GtPath(
                      span: GtSpan(4, 10),
                      id: GtPathModuleId(
                        span: GtSpan(4, 10),
                        module_id: GtModuleId("order"),
                      ),
                      path: "./book",
                    ),
                    reference: Name(GtSpan(11, 15), GtIdentifier(GtSpan(11, 15), "Book")),
                  ),
                ],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("order"), "Order"),
                    span: GtSpan(17, 65),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(17, 22), "Order"),
                    descriptor: Object(GtObject(
                      span: GtSpan(24, 65),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(17, 22), "Order")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(28, 45),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(28, 32), "user"),
                          descriptor: InlineImport(GtInlineImport(
                            span: GtSpan(34, 45),
                            doc: None,
                            attributes: [],
                            name: GtIdentifier(GtSpan(41, 45), "User"),
                            path: GtPath(
                              span: GtSpan(34, 40),
                              id: GtPathModuleId(
                                span: GtSpan(34, 40),
                                module_id: GtModuleId("order"),
                              ),
                              path: "./user",
                            ),
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(49, 62),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(49, 54), "books"),
                          descriptor: Array(GtArray(
                            span: GtSpan(56, 62),
                            doc: None,
                            attributes: [],
                            descriptor: Reference(GtReference(
                              span: GtSpan(57, 61),
                              doc: None,
                              attributes: [],
                              id: GtReferenceId(GtModuleId("order"), GtSpan(57, 61)),
                              identifier: GtIdentifier(GtSpan(57, 61), "Book"),
                            )),
                          )),
                          required: true,
                        ),
                      ],
                    )),
                  ),
                ],
              ),
              resolve: GtpModuleResolve(
                paths: {
                  GtPath(
                    span: GtSpan(4, 10),
                    id: GtPathModuleId(
                      span: GtSpan(4, 10),
                      module_id: GtModuleId("order"),
                    ),
                    path: "./book",
                  ): "book.type",
                  GtPath(
                    span: GtSpan(34, 40),
                    id: GtPathModuleId(
                      span: GtSpan(34, 40),
                      module_id: GtModuleId("order"),
                    ),
                    path: "./user",
                  ): "user.type",
                },
                identifiers: {
                  GtIdentifier(GtSpan(57, 61), "Book"): GtpModuleIdentifierResolve(
                    source: External(GtPath(
                      span: GtSpan(4, 10),
                      id: GtPathModuleId(
                        span: GtSpan(4, 10),
                        module_id: GtModuleId("order"),
                      ),
                      path: "./book",
                    )),
                  ),
                },
                definitions: {
                  GtDefinitionId(GtModuleId("book"), "Book"): GtProjectModuleDefinitionResolve(
                    references: [
                      GtReferenceId(GtModuleId("order"), GtSpan(57, 61)),
                    ],
                    deps: [],
                  ),
                },
                reference_definition_ids: {
                  GtReferenceId(GtModuleId("order"), GtSpan(57, 61)): GtDefinitionId(GtModuleId("book"), "Book"),
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
              module: GtModule(
                id: GtModuleId("user"),
                doc: None,
                imports: [],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("user"), "User"),
                    span: GtSpan(0, 42),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(0, 4), "User"),
                    descriptor: Object(GtObject(
                      span: GtSpan(6, 42),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(0, 4), "User")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(10, 23),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(10, 15), "email"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(17, 23),
                            kind: String,
                            doc: None,
                            attributes: [],
                          )),
                          required: true,
                        ),
                        GtProperty(
                          span: GtSpan(27, 39),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(27, 31), "name"),
                          descriptor: Primitive(GtPrimitive(
                            span: GtSpan(33, 39),
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
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
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
            version: None,
            root: "examples/basic",
            out: "dist",
            src: "",
            entry: "order.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "mode": types,
              "prefer": interface,
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "manager": poetry,
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
        let config = GtpConfig::from_entry("module", "./examples/process", "anonymous.type");
        let project = GtProject::load("genotype.toml".into(), config);
        assert_ron_snapshot!(
            project.unwrap(),
            @r#"
        GtProject(
          modules: [
            GtProjectModule(
              path: "anonymous.type",
              module: GtModule(
                id: GtModuleId("anonymous"),
                doc: None,
                imports: [],
                aliases: [
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("anonymous"), "Order"),
                    span: GtSpan(0, 91),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(0, 5), "Order"),
                    descriptor: Object(GtObject(
                      span: GtSpan(7, 91),
                      doc: None,
                      attributes: [],
                      name: Named(GtIdentifier(GtSpan(0, 5), "Order")),
                      extensions: [],
                      properties: [
                        GtProperty(
                          span: GtSpan(11, 89),
                          doc: None,
                          attributes: [],
                          name: GtKey(GtSpan(11, 19), "delivery"),
                          descriptor: Object(GtObject(
                            span: GtSpan(21, 89),
                            doc: None,
                            attributes: [],
                            name: Alias(GtIdentifier(GtSpan(21, 89), "OrderDelivery"), Property(GtIdentifier(GtSpan(0, 5), "Order"), [
                              GtKey(GtSpan(11, 19), "delivery"),
                            ])),
                            extensions: [],
                            properties: [
                              GtProperty(
                                span: GtSpan(27, 85),
                                doc: None,
                                attributes: [],
                                name: GtKey(GtSpan(27, 34), "address"),
                                descriptor: Object(GtObject(
                                  span: GtSpan(36, 85),
                                  doc: None,
                                  attributes: [],
                                  name: Alias(GtIdentifier(GtSpan(36, 85), "OrderDeliveryAddress"), Property(GtIdentifier(GtSpan(0, 5), "Order"), [
                                    GtKey(GtSpan(11, 19), "delivery"),
                                    GtKey(GtSpan(27, 34), "address"),
                                  ])),
                                  extensions: [],
                                  properties: [
                                    GtProperty(
                                      span: GtSpan(44, 58),
                                      doc: None,
                                      attributes: [],
                                      name: GtKey(GtSpan(44, 50), "street"),
                                      descriptor: Primitive(GtPrimitive(
                                        span: GtSpan(52, 58),
                                        kind: String,
                                        doc: None,
                                        attributes: [],
                                      )),
                                      required: true,
                                    ),
                                    GtProperty(
                                      span: GtSpan(66, 78),
                                      doc: None,
                                      attributes: [],
                                      name: GtKey(GtSpan(66, 70), "city"),
                                      descriptor: Primitive(GtPrimitive(
                                        span: GtSpan(72, 78),
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
                  GtAlias(
                    id: GtDefinitionId(GtModuleId("anonymous"), "Email"),
                    span: GtSpan(93, 145),
                    doc: None,
                    attributes: [],
                    name: GtIdentifier(GtSpan(93, 98), "Email"),
                    descriptor: Union(GtUnion(
                      span: GtSpan(100, 145),
                      doc: None,
                      attributes: [],
                      descriptors: [
                        Primitive(GtPrimitive(
                          span: GtSpan(100, 106),
                          kind: String,
                          doc: None,
                          attributes: [],
                        )),
                        Object(GtObject(
                          span: GtSpan(109, 145),
                          doc: None,
                          attributes: [],
                          name: Alias(GtIdentifier(GtSpan(109, 145), "EmailObj"), Alias(GtIdentifier(GtSpan(93, 98), "Email"))),
                          extensions: [],
                          properties: [
                            GtProperty(
                              span: GtSpan(113, 125),
                              doc: None,
                              attributes: [],
                              name: GtKey(GtSpan(113, 117), "name"),
                              descriptor: Primitive(GtPrimitive(
                                span: GtSpan(119, 125),
                                kind: String,
                                doc: None,
                                attributes: [],
                              )),
                              required: true,
                            ),
                            GtProperty(
                              span: GtSpan(129, 142),
                              doc: None,
                              attributes: [],
                              name: GtKey(GtSpan(129, 134), "email"),
                              descriptor: Primitive(GtPrimitive(
                                span: GtSpan(136, 142),
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
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
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
            version: None,
            root: "examples/process",
            out: "dist",
            src: "",
            entry: "anonymous.type",
            ts: {
              "enabled": false,
              "out": "ts",
              "manifest": {},
              "dependencies": {},
              "mode": types,
              "prefer": interface,
              "tsconfig": TsConfigLangTsconfig(
                allowImportingTsExtensions: false,
              ),
            },
            py: {
              "module": PyModuleName("module"),
              "version": latest,
              "manager": poetry,
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

    #[test]
    fn test_error_undefined_inline_import_type() {
        let config = GtpConfig::from_entry(
            "module",
            "./examples/errors/undefined-inline",
            "collection.type",
        );
        let error = GtProject::load("genotype.toml".into(), config).unwrap_err();
        assert_debug_snapshot!(
          error,
          @r#"
        UndefinedType {
            span: GtSpan(
                44,
                59,
            ),
            identifier: "PackgesSettings",
        }
        "#
        );
    }

    fn basic_config() -> GtpConfig {
        GtpConfig::from_root("module", "./examples/basic")
    }
}
