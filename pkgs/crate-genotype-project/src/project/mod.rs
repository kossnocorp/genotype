use crate::prelude::internal::*;

// region: Modules

mod paths;
pub use paths::*;

mod resolve;
pub use resolve::*;

mod pkg;

// endregion

/// Genotype project. Represents configuration with currently loaded modules.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProject {
    /// Known project modules mapped by their workspace path.
    pub modules: IndexMap<GtpModulePath, GtpModule>,
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
            modules: IndexMap::new(),
            config,
            paths,
        })
    }

    /// Tries to initialize a module in the project. If the module already initialized, it resolves
    /// none signifying that the module is already processing or loaded. Otherwise, it initializes
    /// the module and returns some [GtModuleId].
    pub fn init_module(&mut self, path: &GtpModulePath) -> Result<Option<GtModuleId>> {
        match self.has_module(path) {
            true => Ok(None),
            false => {
                self.modules.insert(path.clone(), GtpModule::Initialized);
                let module_id = path.to_module_id(&self.paths.src)?;
                Ok(Some(module_id))
            }
        }
    }

    /// Checks if the module is already initialized in the project.
    pub fn has_module(&self, path: &GtpModulePath) -> bool {
        self.modules.contains_key(path)
    }

    /// Sets the state of a module in the project.
    pub fn set_module(&mut self, path: &GtpModulePath, module_state: GtpModule) {
        self.modules.insert(path.clone(), module_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                      GtPath(
                        span: GtSpan(4, 12),
                        id: GtPathModuleId(
                          span: GtSpan(4, 12),
                          module_id: GtModuleId("book"),
                        ),
                        path: "./author",
                      ),
                    ],
                    exports: [
                      GtIdentifier(GtSpan(21, 25), "Book"),
                    ],
                    references: [
                      GtIdentifier(GtSpan(56, 62), "Author"),
                    ],
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
                      GtPath(
                        span: GtSpan(4, 10),
                        id: GtPathModuleId(
                          span: GtSpan(4, 10),
                          module_id: GtModuleId("order"),
                        ),
                        path: "./book",
                      ),
                      GtPath(
                        span: GtSpan(34, 41),
                        id: GtPathModuleId(
                          span: GtSpan(34, 41),
                          module_id: GtModuleId("order"),
                        ),
                        path: "./user",
                      ),
                    ],
                    exports: [
                      GtIdentifier(GtSpan(17, 22), "Order"),
                    ],
                    references: [
                      GtIdentifier(GtSpan(57, 61), "Book"),
                    ],
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
              "dist": "ts",
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
              "dist": "py",
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
              "dist": "rs",
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
              "dist": "ts",
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
              "dist": "py",
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
              "dist": "rs",
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
            "examples/errors/undefined-inline/src/collection.type": Error(Resolve(
              path: "examples/errors/undefined-inline/src/collection.type",
              error: UndefinedType(
                span: GtSpan(44, 59),
                identifier: "PackgesSettings",
              ),
            )),
            "examples/errors/undefined-inline/src/package.type": Resolved(GtpModuleResolved(
              project_module_parse: GtpModuleParse(
                path: "examples/errors/undefined-inline/src/package.type",
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
              "dist": "ts",
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
              "dist": "py",
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
              "dist": "rs",
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
