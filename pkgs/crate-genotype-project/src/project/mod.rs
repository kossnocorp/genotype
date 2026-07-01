use crate::prelude::internal::*;

// region: Modules

mod resolve;
pub use resolve::*;

mod diagnostics;

mod pkg;

mod sources;

// endregion

pub const DEFAULT_PROJECT_NAME: &str = "types";

/// Genotype project. Represents configuration with currently loaded modules.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProject {
    /// Known project modules mapped by their workspace path.
    modules: IndexMap<GtpModulePath, GtpModule>,

    /// Known module sources.
    // TODO: It must rebuild when a module is changed.
    module_sources: IndexMap<GtpModulePath, IndexSet<GtpModuleSource>>,

    /// Project name resolved from config, config parent directory, or default.
    name: String,

    /// Project configuration.
    config: GtpConfig,

    /// Project paths.
    paths: GtpPaths,
}

impl GtProject {
    pub fn try_new(
        fallback_name: String,
        config_file_path: GtpConfigFilePath,
        config: GtpConfig,
    ) -> Result<Self> {
        let paths = GtProject::try_new_paths(config_file_path, &config)
            .wrap_err("failed to initialize project paths from config")?;
        let name = config.name.clone().unwrap_or(fallback_name);

        Ok(Self {
            modules: IndexMap::new(),
            module_sources: IndexMap::new(),
            name,
            config,
            paths,
        })
    }

    fn try_new_paths(config_file_path: GtpConfigFilePath, config: &GtpConfig) -> Result<GtpPaths> {
        let config_dir = config_file_path.to_config_dir_path();
        let root = config.root.to_cwd_relative_path(&config_dir).into();
        let dist = config.dist.to_cwd_relative_path(&root).into();
        let src = config.src.to_cwd_relative_path(&root).into();
        let entry = config.entry.to_cwd_relative_path(&src).into();

        Ok(GtpPaths {
            config_file: config_file_path,
            root,
            dist,
            src,
            entry,
        })
    }

    pub fn modules(&self) -> &IndexMap<GtpModulePath, GtpModule> {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut IndexMap<GtpModulePath, GtpModule> {
        &mut self.modules
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> &GtpConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut GtpConfig {
        &mut self.config
    }

    pub fn paths(&self) -> &GtpPaths {
        &self.paths
    }

    /// Tries to initialize a module in the project. If the module already initialized, it resolves
    /// none signifying that the module is already processing or loaded. Otherwise, it initializes
    /// the module and returns some [GtModuleId].
    pub fn init_module(&mut self, source: &GtpModuleSource) -> Result<Option<GtModuleId>> {
        let path = source.path();
        match self.has_module(path) {
            true => Ok(None),
            false => {
                self.modules
                    .insert(path.clone(), GtpModule::Initialized(source.clone()));
                let module_id = path.to_module_id(&self.paths.src)?;
                Ok(Some(module_id))
            }
        }
    }

    /// Checks if the module is already initialized in the project.
    fn has_module(&self, path: &GtpModulePath) -> bool {
        self.modules.contains_key(path)
    }

    /// Sets the state of a module in the project.
    pub fn set_module(&mut self, path: &GtpModulePath, module_state: GtpModule) {
        self.modules.insert(path.clone(), module_state);
    }

    pub fn lang_enabled(&self, lang: GtLang) -> bool {
        self.config.lang_enabled(lang)
    }

    pub fn lang_config(&self, lang: GtLang) -> &dyn GtpLangConfig {
        self.config.lang(lang)
    }

    pub fn lang_package_enabled(&self, lang_config: &dyn GtpLangConfig) -> bool {
        self.config.lang_package_enabled(lang_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uses_config_name() {
        let config = GtpConfig::parse(r#"name = "example-package""#.into()).unwrap();
        let project =
            GtProject::try_new("fallback-name".into(), "genotype.toml".into(), config).unwrap();
        assert_equal!(project.name, "example-package");
    }

    #[test]
    fn test_uses_fallback_name() {
        let project = GtProject::try_new(
            "fallback-name".into(),
            "genotype.toml".into(),
            GtpConfig::default(),
        )
        .unwrap();
        assert_equal!(project.name, "fallback-name");
    }

    #[test]
    fn test_glob() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        assert_ron_snapshot!(project, @r#"
        GtProject(
          modules: {
            "examples/basic/src/author.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/basic/src/author.type",
                source: Entry(
                  path: "examples/basic/src/author.type",
                ),
                source_code: "Author: {\n  name: string,\n}",
                module_parse: GtModuleParse(
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
                        generics: [],
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
                  resolve: GtModuleResolve(
                    deps: [],
                    exports: [
                      GtIdentifier(GtSpan(0, 6), "Author"),
                    ],
                    references: [],
                    generic_parameters: [],
                  ),
                ),
              ),
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
            )),
            "examples/basic/src/book.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/basic/src/book.type",
                source: Entry(
                  path: "examples/basic/src/book.type",
                ),
                source_code: "use ./author/Author\n\nBook: {\n  title: string,\n  author: Author,\n}",
                module_parse: GtModuleParse(
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
                        generics: [],
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
                                arguments: [],
                              )),
                              required: true,
                            ),
                          ],
                        )),
                      ),
                    ],
                  ),
                  resolve: GtModuleResolve(
                    deps: [
                      GtModuleSource(
                        span: GtSpan(0, 19),
                        path: GtPath(
                          span: GtSpan(4, 12),
                          id: GtPathModuleId(
                            span: GtSpan(4, 12),
                            module_id: GtModuleId("book"),
                          ),
                          path: "./author",
                        ),
                      ),
                    ],
                    exports: [
                      GtIdentifier(GtSpan(21, 25), "Book"),
                    ],
                    references: [
                      GtIdentifier(GtSpan(56, 62), "Author"),
                    ],
                    generic_parameters: [],
                  ),
                ),
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
                  GtIdentifier(GtSpan(56, 62), "Author"): GtpModuleResolveIdentifier(
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
                  GtDefinitionId(GtModuleId("author"), "Author"): GtpModuleResolveDefinition(
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
            )),
            "examples/basic/src/order.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/basic/src/order.type",
                source: Entry(
                  path: "examples/basic/src/order.type",
                ),
                source_code: "use ./book/Book\n\nOrder: {\n  user: ./user/User,\n  books: [Book],\n}",
                module_parse: GtModuleParse(
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
                        generics: [],
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
                                arguments: [],
                                path: GtPath(
                                  span: GtSpan(34, 41),
                                  id: GtPathModuleId(
                                    span: GtSpan(34, 41),
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
                                  arguments: [],
                                )),
                              )),
                              required: true,
                            ),
                          ],
                        )),
                      ),
                    ],
                  ),
                  resolve: GtModuleResolve(
                    deps: [
                      GtModuleSource(
                        span: GtSpan(0, 15),
                        path: GtPath(
                          span: GtSpan(4, 10),
                          id: GtPathModuleId(
                            span: GtSpan(4, 10),
                            module_id: GtModuleId("order"),
                          ),
                          path: "./book",
                        ),
                      ),
                      GtModuleSource(
                        span: GtSpan(34, 45),
                        path: GtPath(
                          span: GtSpan(34, 41),
                          id: GtPathModuleId(
                            span: GtSpan(34, 41),
                            module_id: GtModuleId("order"),
                          ),
                          path: "./user",
                        ),
                      ),
                    ],
                    exports: [
                      GtIdentifier(GtSpan(17, 22), "Order"),
                    ],
                    references: [
                      GtIdentifier(GtSpan(57, 61), "Book"),
                    ],
                    generic_parameters: [],
                  ),
                ),
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
                    span: GtSpan(34, 41),
                    id: GtPathModuleId(
                      span: GtSpan(34, 41),
                      module_id: GtModuleId("order"),
                    ),
                    path: "./user",
                  ): "user.type",
                },
                identifiers: {
                  GtIdentifier(GtSpan(57, 61), "Book"): GtpModuleResolveIdentifier(
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
                  GtDefinitionId(GtModuleId("book"), "Book"): GtpModuleResolveDefinition(
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
            )),
            "examples/basic/src/user.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/basic/src/user.type",
                source: Entry(
                  path: "examples/basic/src/user.type",
                ),
                source_code: "User: {\n  email: string,\n  name: string,\n}",
                module_parse: GtModuleParse(
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
                        generics: [],
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
                  resolve: GtModuleResolve(
                    deps: [],
                    exports: [
                      GtIdentifier(GtSpan(0, 4), "User"),
                    ],
                    references: [],
                    generic_parameters: [],
                  ),
                ),
              ),
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
            )),
          },
          module_sources: {
            "examples/basic/src/author.type": [
              Dependency(
                path: "examples/basic/src/author.type",
                parent_path: "examples/basic/src/book.type",
                parent_span: GtSpan(0, 19),
              ),
              Entry(
                path: "examples/basic/src/author.type",
              ),
            ],
            "examples/basic/src/book.type": [
              Dependency(
                path: "examples/basic/src/book.type",
                parent_path: "examples/basic/src/order.type",
                parent_span: GtSpan(0, 15),
              ),
              Entry(
                path: "examples/basic/src/book.type",
              ),
            ],
            "examples/basic/src/order.type": [
              Entry(
                path: "examples/basic/src/order.type",
              ),
            ],
            "examples/basic/src/user.type": [
              Dependency(
                path: "examples/basic/src/user.type",
                parent_path: "examples/basic/src/order.type",
                parent_span: GtSpan(34, 45),
              ),
              Entry(
                path: "examples/basic/src/user.type",
              ),
            ],
          },
          name: "basic",
          config: GtpConfig(
            name: None,
            version: None,
            package: true,
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
            },
          ),
          paths: GtpPaths(
            config_file: "examples/basic/genotype.toml",
            root: "examples/basic",
            dist: "examples/basic/dist",
            src: "examples/basic/src",
            entry: "examples/basic/src/**/*.type",
          ),
        )
        "#);
    }

    #[test]
    fn test_process_anonymous() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/process".into(), None).unwrap();
        assert_ron_snapshot!(
            project,
            @r#"
        GtProject(
          modules: {
            "examples/process/src/anonymous.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/process/src/anonymous.type",
                source: Entry(
                  path: "examples/process/src/anonymous.type",
                ),
                source_code: "Order: {\n  delivery: {\n    address: {\n      street: string,\n      city: string,\n    }\n  }\n}\n\nEmail: string | {\n  name: string,\n  email: string,\n}\n\n",
                module_parse: GtModuleParse(
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
                        generics: [],
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
                        generics: [],
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
                  resolve: GtModuleResolve(
                    deps: [],
                    exports: [
                      GtIdentifier(GtSpan(0, 5), "Order"),
                      GtIdentifier(GtSpan(93, 98), "Email"),
                    ],
                    references: [],
                    generic_parameters: [],
                  ),
                ),
              ),
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
            )),
          },
          module_sources: {
            "examples/process/src/anonymous.type": [
              Entry(
                path: "examples/process/src/anonymous.type",
              ),
            ],
          },
          name: "process",
          config: GtpConfig(
            name: None,
            version: None,
            package: true,
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
            },
          ),
          paths: GtpPaths(
            config_file: "examples/process/genotype.toml",
            root: "examples/process",
            dist: "examples/process/dist",
            src: "examples/process/src",
            entry: "examples/process/src/**/*.type",
          ),
        )
        "#
        );
    }

    #[test]
    fn test_error_undefined_inline_import_type() {
        let project = GtpRuntimeSystem::new_and_load_all_modules(
            &"./examples/errors/undefined-inline".into(),
            None,
        )
        .unwrap();
        assert_ron_snapshot!(
          project,
          @r#"
        GtProject(
          modules: {
            "examples/errors/undefined-inline/src/collection.type": Error(Entry(
              path: "examples/errors/undefined-inline/src/collection.type",
            ), Resolve(
              path: "examples/errors/undefined-inline/src/collection.type",
              error: UndefinedType(
                span: GtSpan(44, 59),
                identifier: "PackgesSettings",
                reason: "can\'t find definition for the inline-import reference",
              ),
            )),
            "examples/errors/undefined-inline/src/package.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/errors/undefined-inline/src/package.type",
                source: Entry(
                  path: "examples/errors/undefined-inline/src/package.type",
                ),
                source_code: "PackageSettings: {\n  value: string,\n}\n",
                module_parse: GtModuleParse(
                  module: GtModule(
                    id: GtModuleId("package"),
                    doc: None,
                    imports: [],
                    aliases: [
                      GtAlias(
                        id: GtDefinitionId(GtModuleId("package"), "PackageSettings"),
                        span: GtSpan(0, 37),
                        doc: None,
                        attributes: [],
                        name: GtIdentifier(GtSpan(0, 15), "PackageSettings"),
                        generics: [],
                        descriptor: Object(GtObject(
                          span: GtSpan(17, 37),
                          doc: None,
                          attributes: [],
                          name: Named(GtIdentifier(GtSpan(0, 15), "PackageSettings")),
                          extensions: [],
                          properties: [
                            GtProperty(
                              span: GtSpan(21, 34),
                              doc: None,
                              attributes: [],
                              name: GtKey(GtSpan(21, 26), "value"),
                              descriptor: Primitive(GtPrimitive(
                                span: GtSpan(28, 34),
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
                  resolve: GtModuleResolve(
                    deps: [],
                    exports: [
                      GtIdentifier(GtSpan(0, 15), "PackageSettings"),
                    ],
                    references: [],
                    generic_parameters: [],
                  ),
                ),
              ),
              resolve: GtpModuleResolve(
                paths: {},
                identifiers: {},
                definitions: {},
                reference_definition_ids: {},
              ),
            )),
          },
          module_sources: {
            "examples/errors/undefined-inline/src/collection.type": [
              Entry(
                path: "examples/errors/undefined-inline/src/collection.type",
              ),
            ],
            "examples/errors/undefined-inline/src/package.type": [
              Dependency(
                path: "examples/errors/undefined-inline/src/package.type",
                parent_path: "examples/errors/undefined-inline/src/collection.type",
                parent_span: GtSpan(34, 59),
              ),
              Entry(
                path: "examples/errors/undefined-inline/src/package.type",
              ),
            ],
          },
          name: "undefined-inline",
          config: GtpConfig(
            name: None,
            version: None,
            package: true,
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
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
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
            },
          ),
          paths: GtpPaths(
            config_file: "examples/errors/undefined-inline/genotype.toml",
            root: "examples/errors/undefined-inline",
            dist: "examples/errors/undefined-inline/dist",
            src: "examples/errors/undefined-inline/src",
            entry: "examples/errors/undefined-inline/src/**/*.type",
          ),
        )
        "#
        );
    }
}
