use crate::prelude::internal::*;

/// System project runtime. It combines parallel project loader, file system access and stdio
/// diagnostics reporting. It is the default project runtime used by the CLI.
pub struct GtbSystem {
    /// Current working directory path.
    cwd_path: GtpCwdPath,

    /// Base path for the project source to resolve relative file paths.
    base_path: GtpCwdRelativePath,
}

impl GtbSystem {
    /// Creates a new system backend with the given base path.
    pub fn new(path: &GtpCwdRelativeOrAbsoluteStringPath) -> Result<Self> {
        let cwd_path = GtpCwdPath::try_new()?;

        let base_path = path
            .try_into()
            .wrap_err_with(|| format!("Failed to normalize base path '{path}'"))?;

        Ok(Self {
            cwd_path,
            base_path,
        })
    }
}

impl GtbFsEnv for GtbSystem {
    /// Returns the cwd path.
    fn cwd_path(&self) -> &GtpCwdPath {
        &self.cwd_path
    }

    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath {
        &self.base_path
    }
}

impl GtbFsSourceOs for GtbSystem {}

impl GtbFsSinkSystem for GtbSystem {}

impl GtbDiagnosticSinkStdio for GtbSystem {}

impl GtbFormatterRunnerSystem<GtbDiagnosticSinkStdioKind> for GtbSystem {}

impl GtBackend for GtbSystem {
    type FileSourceKind = GtbFsSourceOsKind;

    type FileSinkKind = GtbFsSinkSystemKind;

    type DiagnosticSinkKind = GtbDiagnosticSinkStdioKind;

    type FormatterRunnerKind = GtbFormatterRunnerSystemKind;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob() {
        let backend = GtbSystem::new(&"./examples/basic".into()).unwrap();
        let project = block_on(backend.create_project_and_load_all_modules(None)).unwrap();

        assert_ron_snapshot!(project, @r#"
        GtProject(
          modules: {
            "examples/basic/src/author.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/basic/src/author.type",
                  source: Entry(
                    path: "examples/basic/src/author.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "Author: {\n  name: string,\n}",
                    hash: GtpSourceCodeHash("a0cc1ef6c66abff6"),
                  ),
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
                  deps: [],
                ),
              ),
            )),
            "examples/basic/src/book.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/basic/src/book.type",
                  source: Entry(
                    path: "examples/basic/src/book.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "use ./author/Author\n\nBook: {\n  title: string,\n  author: Author,\n}",
                    hash: GtpSourceCodeHash("0e1f939f87013925"),
                  ),
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
                  deps: [
                    GtModuleId("author"),
                  ],
                ),
              ),
            )),
            "examples/basic/src/order.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/basic/src/order.type",
                  source: Entry(
                    path: "examples/basic/src/order.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "use ./book/Book\n\nOrder: {\n  user: ./user/User,\n  books: [Book],\n}",
                    hash: GtpSourceCodeHash("41603d029c939314"),
                  ),
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
                  deps: [
                    GtModuleId("book"),
                    GtModuleId("user"),
                  ],
                ),
              ),
            )),
            "examples/basic/src/user.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/basic/src/user.type",
                  source: Entry(
                    path: "examples/basic/src/user.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "User: {\n  email: string,\n  name: string,\n}",
                    hash: GtpSourceCodeHash("ab03677762e127e9"),
                  ),
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
                  deps: [],
                ),
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
            build: GtpBuildConfig(
              file: true,
              cleanup: true,
            ),
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            formatters: [],
            warning_comment: true,
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
              "formatters": [],
              "mode": types,
              "prefer": interface,
              "ext": js,
              "tsconfig": None,
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
              "formatters": [],
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
              "formatters": [],
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
        let backend = GtbSystem::new(&"./examples/process".into()).unwrap();
        let project = block_on(backend.create_project_and_load_all_modules(None)).unwrap();

        assert_ron_snapshot!(
            project,
            @r#"
        GtProject(
          modules: {
            "examples/process/src/anonymous.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/process/src/anonymous.type",
                  source: Entry(
                    path: "examples/process/src/anonymous.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "Order: {\n  delivery: {\n    address: {\n      street: string,\n      city: string,\n    }\n  }\n}\n\nEmail: string | {\n  name: string,\n  email: string,\n}\n\n",
                    hash: GtpSourceCodeHash("4abe27e1aec226ec"),
                  ),
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
                  deps: [],
                ),
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
            build: GtpBuildConfig(
              file: true,
              cleanup: true,
            ),
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            formatters: [],
            warning_comment: true,
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
              "formatters": [],
              "mode": types,
              "prefer": interface,
              "ext": js,
              "tsconfig": None,
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
              "formatters": [],
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
              "formatters": [],
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
        let backend = GtbSystem::new(&"./examples/errors/undefined-inline".into()).unwrap();
        let project = block_on(backend.create_project_and_load_all_modules(None)).unwrap();

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
            "examples/errors/undefined-inline/src/package.type": TypeChecked(GtpModuleTypeChecked(
              module_resolved: GtpModuleResolved(
                project_module_parse: GtpModuleParse(
                  path: "examples/errors/undefined-inline/src/package.type",
                  source: Entry(
                    path: "examples/errors/undefined-inline/src/package.type",
                  ),
                  source_code: GtpSourceCode(
                    content: "PackageSettings: {\n  value: string,\n}\n",
                    hash: GtpSourceCodeHash("fd933c54d5fe32cd"),
                  ),
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
                  deps: [],
                ),
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
            build: GtpBuildConfig(
              file: true,
              cleanup: true,
            ),
            root: "",
            dist: "dist",
            src: "src",
            entry: "**/*.type",
            formatters: [],
            warning_comment: true,
            ts: {
              "enabled": false,
              "dist": None,
              "package": None,
              "manifest": {},
              "dependencies": {},
              "formatters": [],
              "mode": types,
              "prefer": interface,
              "ext": js,
              "tsconfig": None,
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
              "formatters": [],
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
              "formatters": [],
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
