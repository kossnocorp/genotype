use crate::prelude::internal::*;

pub struct RsCompiler<'a> {
    project: &'a GtProject,
    config: GtlConfig<'a, RsConfig>,
}

impl<'project> GtlCompiler<'project> for RsCompiler<'project> {
    type ProjectModule = RsProjectModule;

    type Manifest<'config>
        = RsManifest<'project, 'config>
    where
        'project: 'config;

    fn lang(&self) -> GtLang {
        GtLang::Rs
    }

    fn project(&self) -> &GtProject {
        self.project
    }

    fn config(&self) -> &GtlConfig<'project, RsConfig> {
        &self.config
    }

    fn new(project: &'project GtProject) -> Self {
        let lang_config = &project.config().rs;
        let config = GtlConfig::new(project, lang_config);
        RsCompiler { project, config }
    }

    fn generate_extra_files(
        &self,
        _project: &GtlProject<'project, '_, RsProjectModule>,
    ) -> Option<GtlGenerations<RsProjectModule>> {
        let mut files = vec![];
        let mut diagnostics = vec![];

        let (module_indices, module_indices_diagnostics) = self.generate_module_indices();
        files.extend(module_indices);
        if let Some(module_indices_diagnostics) = module_indices_diagnostics {
            diagnostics.extend(module_indices_diagnostics);
        }

        Some((files, Some(diagnostics)))
    }

    fn gitignore_source_code(&self) -> Option<String> {
        Some("target".into())
    }
}

impl<'project> RsCompiler<'project> {
    fn generate_module_indices(&self) -> GtlGenerations<RsProjectModule> {
        let mut diagnostics = vec![];
        let mut crate_paths: IndexMap<GtpTargetFilePath, IndexSet<String>> = IndexMap::new();

        for module_path in self.project.modules().keys() {
            let target_path = self.config.module_target_file_path(module_path);
            match target_path {
                Ok(target_path) => {
                    let rel_path = target_path
                        .relative_path()
                        .strip_prefix(self.config.pkg_src_path().relative_path())
                        .map_err(|err| miette!("Failed to strip package source path: {err:?}"));

                    let Ok(rel_path) = rel_path else {
                        diagnostics.push(GtDiagnostic::error(format!(
                            "Failed to generate index `mod.rs` files for `{target_path}` (generated from `{module_path}`): {}",
                            rel_path.unwrap_err()
                        )));
                        continue;
                    };

                    let mut module_path = GtpTargetFilePath::new(rel_path.into());

                    loop {
                        let name = module_name(&module_path);
                        let parent_path = module_path.to_parent().unwrap_or_else(|| "".into());

                        crate_paths
                            .entry(parent_path.clone())
                            .and_modify(|paths| {
                                paths.insert(name.clone());
                            })
                            .or_insert_with(|| IndexSet::from_iter(vec![name]));

                        if parent_path == "".into() {
                            break;
                        }

                        module_path = parent_path;
                    }
                }

                Err(err) => {
                    diagnostics.push(GtDiagnostic::error(format!(
                        "Failed to generate index `mod.rs` files for `{module_path}`: {err}",
                    )));
                }
            }
        }

        let generations = crate_paths
            .into_iter()
            .map(|(module_path, modules)| {
                let file_name = if module_path == "".into() {
                    if self.config.package_enabled() {
                        "lib.rs"
                    } else {
                        "mod.rs"
                    }
                } else {
                    "mod.rs"
                };
                let path = module_path.join_relative_path(&file_name.into());
                let path = self
                    .config
                    .pkg_src_file_path(&GtpPkgSrcDirRelativePath::new(
                        path.relative_path().clone(),
                    ));

                let mut source_code = modules
                    .iter()
                    .map(|module| {
                        indoc::formatdoc! {r#"
                            pub(crate) mod {module};
                            pub use {module}::*;
                        "#}
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                source_code += "\n";

                GtlProjectFileExtraGenerated { path, source_code }.into()
            })
            .collect();

        (generations, Some(diagnostics))
    }
}

fn module_name(path: &GtpTargetFilePath) -> String {
    path.relative_path()
        .with_extension("")
        .file_name()
        .unwrap_or_default()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

        assert_ron_snapshot!(
          modules(&project),
          @r#"
        {
          "examples/basic/src/author.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/basic/src/author.type",
              target_path: "examples/basic/dist/rs/src/author.rs",
              project_module: RsProjectModule(
                module: RsModule(
                  id: GtModuleId("author"),
                  doc: None,
                  imports: [
                    RsUse(
                      dependency: Serde,
                      reference: Named([
                        Name(RsIdentifier("Deserialize")),
                        Name(RsIdentifier("Serialize")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Struct(RsStruct(
                      id: GtDefinitionId(GtModuleId("author"), "Author"),
                      doc: None,
                      attributes: [
                        RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                      ],
                      name: RsIdentifier("Author"),
                      generics: [],
                      fields: Resolved([
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("name"),
                          descriptor: Primitive(String),
                        ),
                      ]),
                    )),
                  ],
                ),
                project_resolve: RsProjectModuleResolve(
                  definitions: {},
                ),
              ),
            ),
            resolved_module: RsProjectModule(
              module: RsModule(
                id: GtModuleId("author"),
                doc: None,
                imports: [
                  RsUse(
                    dependency: Serde,
                    reference: Named([
                      Name(RsIdentifier("Deserialize")),
                      Name(RsIdentifier("Serialize")),
                    ]),
                  ),
                ],
                definitions: [
                  Struct(RsStruct(
                    id: GtDefinitionId(GtModuleId("author"), "Author"),
                    doc: None,
                    attributes: [
                      RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                    ],
                    name: RsIdentifier("Author"),
                    generics: [],
                    fields: Resolved([
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("name"),
                        descriptor: Primitive(String),
                      ),
                    ]),
                  )),
                ],
              ),
              project_resolve: RsProjectModuleResolve(
                definitions: {},
              ),
            ),
          )),
          "examples/basic/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/basic/src/book.type",
              target_path: "examples/basic/dist/rs/src/book.rs",
              project_module: RsProjectModule(
                module: RsModule(
                  id: GtModuleId("book"),
                  doc: None,
                  imports: [
                    RsUse(
                      dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                      reference: Named([
                        Name(RsIdentifier("Author")),
                      ]),
                    ),
                    RsUse(
                      dependency: Serde,
                      reference: Named([
                        Name(RsIdentifier("Deserialize")),
                        Name(RsIdentifier("Serialize")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Struct(RsStruct(
                      id: GtDefinitionId(GtModuleId("book"), "Book"),
                      doc: None,
                      attributes: [
                        RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                      ],
                      name: RsIdentifier("Book"),
                      generics: [],
                      fields: Resolved([
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("title"),
                          descriptor: Primitive(String),
                        ),
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("author"),
                          descriptor: Reference(RsReference(
                            id: GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                            identifier: RsIdentifier("Author"),
                            arguments: [],
                            definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                          )),
                        ),
                      ]),
                    )),
                  ],
                ),
                project_resolve: RsProjectModuleResolve(
                  definitions: {
                    GtDefinitionId(GtModuleId("author"), "Author"): GtpModuleResolveDefinition(
                      references: [
                        GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                      ],
                      deps: [],
                    ),
                  },
                ),
              ),
            ),
            resolved_module: RsProjectModule(
              module: RsModule(
                id: GtModuleId("book"),
                doc: None,
                imports: [
                  RsUse(
                    dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                    reference: Named([
                      Name(RsIdentifier("Author")),
                    ]),
                  ),
                  RsUse(
                    dependency: Serde,
                    reference: Named([
                      Name(RsIdentifier("Deserialize")),
                      Name(RsIdentifier("Serialize")),
                    ]),
                  ),
                ],
                definitions: [
                  Struct(RsStruct(
                    id: GtDefinitionId(GtModuleId("book"), "Book"),
                    doc: None,
                    attributes: [
                      RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                    ],
                    name: RsIdentifier("Book"),
                    generics: [],
                    fields: Resolved([
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("title"),
                        descriptor: Primitive(String),
                      ),
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("author"),
                        descriptor: Reference(RsReference(
                          id: GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                          identifier: RsIdentifier("Author"),
                          arguments: [],
                          definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                        )),
                      ),
                    ]),
                  )),
                ],
              ),
              project_resolve: RsProjectModuleResolve(
                definitions: {
                  GtDefinitionId(GtModuleId("author"), "Author"): GtpModuleResolveDefinition(
                    references: [
                      GtReferenceId(GtModuleId("book"), GtSpan(56, 62)),
                    ],
                    deps: [],
                  ),
                },
              ),
            ),
          )),
        }
        "#
        );
    }

    #[test]
    fn test_convert_glob() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/glob".into(), None).unwrap();

        assert_ron_snapshot!(
          modules(&project),
          @r#"
        {
          "examples/glob/src/author.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/glob/src/author.type",
              target_path: "examples/glob/dist/rs/src/author.rs",
              project_module: RsProjectModule(
                module: RsModule(
                  id: GtModuleId("author"),
                  doc: None,
                  imports: [
                    RsUse(
                      dependency: Serde,
                      reference: Named([
                        Name(RsIdentifier("Deserialize")),
                        Name(RsIdentifier("Serialize")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Struct(RsStruct(
                      id: GtDefinitionId(GtModuleId("author"), "Author"),
                      doc: None,
                      attributes: [
                        RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                      ],
                      name: RsIdentifier("Author"),
                      generics: [],
                      fields: Resolved([
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("name"),
                          descriptor: Reference(RsReference(
                            id: GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                            identifier: RsIdentifier("AuthorName"),
                            arguments: [],
                            definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                          )),
                        ),
                      ]),
                    )),
                    Alias(RsAlias(
                      id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                      doc: None,
                      name: RsIdentifier("AuthorName"),
                      generics: [],
                      descriptor: Primitive(String),
                    )),
                  ],
                ),
                project_resolve: RsProjectModuleResolve(
                  definitions: {
                    GtDefinitionId(GtModuleId("author"), "AuthorName"): GtpModuleResolveDefinition(
                      references: [
                        GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                      ],
                      deps: [],
                    ),
                  },
                ),
              ),
            ),
            resolved_module: RsProjectModule(
              module: RsModule(
                id: GtModuleId("author"),
                doc: None,
                imports: [
                  RsUse(
                    dependency: Serde,
                    reference: Named([
                      Name(RsIdentifier("Deserialize")),
                      Name(RsIdentifier("Serialize")),
                    ]),
                  ),
                ],
                definitions: [
                  Struct(RsStruct(
                    id: GtDefinitionId(GtModuleId("author"), "Author"),
                    doc: None,
                    attributes: [
                      RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                    ],
                    name: RsIdentifier("Author"),
                    generics: [],
                    fields: Resolved([
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("name"),
                        descriptor: Reference(RsReference(
                          id: GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                          identifier: RsIdentifier("AuthorName"),
                          arguments: [],
                          definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                        )),
                      ),
                    ]),
                  )),
                  Alias(RsAlias(
                    id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                    doc: None,
                    name: RsIdentifier("AuthorName"),
                    generics: [],
                    descriptor: Primitive(String),
                  )),
                ],
              ),
              project_resolve: RsProjectModuleResolve(
                definitions: {
                  GtDefinitionId(GtModuleId("author"), "AuthorName"): GtpModuleResolveDefinition(
                    references: [
                      GtReferenceId(GtModuleId("author"), GtSpan(18, 28)),
                    ],
                    deps: [],
                  ),
                },
              ),
            ),
          )),
          "examples/glob/src/book.type": Resolved(GtlProjectModuleResolved(
            converted: GtlProjectModuleConverted(
              source_path: "examples/glob/src/book.type",
              target_path: "examples/glob/dist/rs/src/book.rs",
              project_module: RsProjectModule(
                module: RsModule(
                  id: GtModuleId("book"),
                  doc: None,
                  imports: [
                    RsUse(
                      dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                      reference: Module,
                    ),
                    RsUse(
                      dependency: Serde,
                      reference: Named([
                        Name(RsIdentifier("Deserialize")),
                        Name(RsIdentifier("Serialize")),
                      ]),
                    ),
                  ],
                  definitions: [
                    Struct(RsStruct(
                      id: GtDefinitionId(GtModuleId("book"), "Book"),
                      doc: None,
                      attributes: [
                        RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                      ],
                      name: RsIdentifier("Book"),
                      generics: [],
                      fields: Resolved([
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("title"),
                          descriptor: Primitive(String),
                        ),
                        RsField(
                          doc: None,
                          attributes: [],
                          name: RsFieldName("author"),
                          descriptor: Reference(RsReference(
                            id: GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                            identifier: RsIdentifier("author.Author"),
                            arguments: [],
                            definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                          )),
                        ),
                        RsField(
                          doc: None,
                          attributes: [
                            RsAttribute("serde(rename = \"authorName\")"),
                          ],
                          name: RsFieldName("author_name"),
                          descriptor: Reference(RsReference(
                            id: GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                            identifier: RsIdentifier("author.AuthorName"),
                            arguments: [],
                            definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                          )),
                        ),
                      ]),
                    )),
                  ],
                ),
                project_resolve: RsProjectModuleResolve(
                  definitions: {
                    GtDefinitionId(GtModuleId("author"), "Author"): GtpModuleResolveDefinition(
                      references: [
                        GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                      ],
                      deps: [],
                    ),
                    GtDefinitionId(GtModuleId("author"), "AuthorName"): GtpModuleResolveDefinition(
                      references: [
                        GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                      ],
                      deps: [],
                    ),
                  },
                ),
              ),
            ),
            resolved_module: RsProjectModule(
              module: RsModule(
                id: GtModuleId("book"),
                doc: None,
                imports: [
                  RsUse(
                    dependency: Local(RsPath(GtModuleId("author"), "super::author")),
                    reference: Module,
                  ),
                  RsUse(
                    dependency: Serde,
                    reference: Named([
                      Name(RsIdentifier("Deserialize")),
                      Name(RsIdentifier("Serialize")),
                    ]),
                  ),
                ],
                definitions: [
                  Struct(RsStruct(
                    id: GtDefinitionId(GtModuleId("book"), "Book"),
                    doc: None,
                    attributes: [
                      RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
                    ],
                    name: RsIdentifier("Book"),
                    generics: [],
                    fields: Resolved([
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("title"),
                        descriptor: Primitive(String),
                      ),
                      RsField(
                        doc: None,
                        attributes: [],
                        name: RsFieldName("author"),
                        descriptor: Reference(RsReference(
                          id: GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                          identifier: RsIdentifier("author.Author"),
                          arguments: [],
                          definition_id: GtDefinitionId(GtModuleId("author"), "Author"),
                        )),
                      ),
                      RsField(
                        doc: None,
                        attributes: [
                          RsAttribute("serde(rename = \"authorName\")"),
                        ],
                        name: RsFieldName("author_name"),
                        descriptor: Reference(RsReference(
                          id: GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                          identifier: RsIdentifier("author.AuthorName"),
                          arguments: [],
                          definition_id: GtDefinitionId(GtModuleId("author"), "AuthorName"),
                        )),
                      ),
                    ]),
                  )),
                ],
              ),
              project_resolve: RsProjectModuleResolve(
                definitions: {
                  GtDefinitionId(GtModuleId("author"), "Author"): GtpModuleResolveDefinition(
                    references: [
                      GtReferenceId(GtModuleId("book"), GtSpan(51, 57)),
                    ],
                    deps: [],
                  ),
                  GtDefinitionId(GtModuleId("author"), "AuthorName"): GtpModuleResolveDefinition(
                    references: [
                      GtReferenceId(GtModuleId("book"), GtSpan(73, 83)),
                    ],
                    deps: [],
                  ),
                },
              ),
            ),
          )),
        }
        "#
        );
    }

    #[test]
    fn test_render() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();

        let dist = compile(&project);

        assert_equal!(dist.files.len(), 5);

        assert_debug_snapshot!(
          dist.files[0].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/basic/dist/rs/.gitignore",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source_code(),
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/basic/dist/rs/Cargo.toml",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source_code(),
          @r#"
        [package]
        edition = "2024"
        name = "basic"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/basic/dist/rs/src/author.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source_code(),
          @"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Author {
            pub name: String,
        }
        "
        );

        assert_debug_snapshot!(
          dist.files[3].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/basic/dist/rs/src/book.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source_code(),
          @"
        use super::author::Author;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Book {
            pub title: String,
            pub author: Author,
        }
        "
        );

        assert_ron_snapshot!(
          dist.files[4].path(),
          @r#""examples/basic/dist/rs/src/lib.rs""#
        );
        assert_snapshot!(
          dist.files[4].source_code(),
          @"
        pub(crate) mod author;
        pub use author::*;

        pub(crate) mod book;
        pub use book::*;
        "
        );
    }

    #[test]
    fn test_render_nested() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/nested".into(), None).unwrap();

        let dist = compile(&project);

        assert_equal!(dist.files.len(), 7);

        assert_debug_snapshot!(
          dist.files[0].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/.gitignore",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source_code(),
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/Cargo.toml",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source_code(),
          @r#"
        [package]
        edition = "2024"
        name = "nested"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/src/inventory.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source_code(),
          @"
        use super::shop::goods::book::Book;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Inventory {
            pub goods: Vec<Book>,
        }
        "
        );

        assert_debug_snapshot!(
          dist.files[3].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/src/lib.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source_code(),
          @"
        pub(crate) mod inventory;
        pub use inventory::*;

        pub(crate) mod shop;
        pub use shop::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[4].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/src/shop/goods/book.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[4].source_code(),
          @"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Book {
            pub title: String,
        }
        "
        );

        assert_debug_snapshot!(
          dist.files[5].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/nested/dist/rs/src/shop/goods/mod.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[5].source_code(),
          @"
        pub(crate) mod book;
        pub use book::*;
        "
        );

        assert_ron_snapshot!(
          dist.files[6].path(),
          @r#""examples/nested/dist/rs/src/shop/mod.rs""#
        );
        assert_snapshot!(
          dist.files[6].source_code(),
          @"
        pub(crate) mod goods;
        pub use goods::*;
        "
        );
    }

    #[test]
    fn test_render_recursive_box() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/recursive".into(), None)
                .unwrap();

        let dist = compile(&project);
        let node_file = dist
            .files
            .iter()
            .find(|file| file.path().as_str().contains("src/node.rs"))
            .unwrap();

        assert_snapshot!(
            node_file.source_code(),
            @r#"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Node {
            pub value: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub next: Option<Box<Node>>,
        }
        "#
        );
    }

    #[test]
    fn test_render_recursive_box_with_extensions() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/recursive".into(), None)
                .unwrap();

        let dist = compile(&project);
        let tree_file = dist
            .files
            .iter()
            .find(|file| file.path().as_str().contains("src/tree.rs"))
            .unwrap();

        assert_snapshot!(
            tree_file.source_code(),
            @r#"
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct NodeMeta {
            pub id: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreeLinkFields {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub parent: Option<TreeNode>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreeNode {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub parent: Option<Box<TreeNode>>,
            pub payload: Box<TreePayload>,
            pub children: Vec<TreeNode>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TreePayload {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub owner: Option<Box<TreeNode>>,
            pub kind: String,
        }
        "#
        );
    }

    #[test]
    fn test_render_extensions() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/extensions".into(), None)
                .unwrap();

        let dist = compile(&project);

        assert_equal!(dist.files.len(), 6);

        assert_debug_snapshot!(
          dist.files[0].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/extensions/dist/rs/.gitignore",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source_code(),
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/extensions/dist/rs/Cargo.toml",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source_code(),
          @r#"
        [package]
        edition = "2024"
        name = "extensions"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        litty = "0.5"
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/extensions/dist/rs/src/admin.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source_code(),
          @r#"
        use serde::{Deserialize, Serialize};
        use litty::serde_literals;
        use crate::named::Name;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Admin {
            pub name: Name,
            pub email: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub age: Option<i64>,
            pub role: AdminRole,
        }

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum AdminRole {
            #[literal("superadmin")]
            Superadmin,
            #[literal("admin")]
            Admin,
            #[literal("moderator")]
            Moderator,
        }
        "#
        );

        assert_debug_snapshot!(
          dist.files[3].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/extensions/dist/rs/src/lib.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[3].source_code(),
          @"
        pub(crate) mod admin;
        pub use admin::*;

        pub(crate) mod named;
        pub use named::*;

        pub(crate) mod user;
        pub use user::*;
        "
        );

        assert_debug_snapshot!(
          dist.files[4].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/extensions/dist/rs/src/named.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[4].source_code(),
          @"
        use litty::serde_literals;
        use serde::{Deserialize, Serialize};

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[literals(named = true)]
        pub struct Named {
            pub name: Name,
        }

        pub type Name = String;
        "
        );

        assert_ron_snapshot!(
          dist.files[5].path(),
          @r#""examples/extensions/dist/rs/src/user.rs""#
        );
        assert_snapshot!(
          dist.files[5].source_code(),
          @r#"
        use super::named::Name;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct User {
            pub name: Name,
            pub email: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub age: Option<i64>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Account {
            pub email: String,
        }
        "#
        );
    }

    #[test]
    fn test_render_dependencies() {
        let project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/dependencies".into(), None)
                .unwrap();

        let dist = compile(&project);

        assert_equal!(dist.files.len(), 4);

        assert_debug_snapshot!(
          dist.files[0].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/dependencies/dist/rs/.gitignore",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[0].source_code(),
          @"target"
        );

        assert_debug_snapshot!(
          dist.files[1].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/dependencies/dist/rs/Cargo.toml",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[1].source_code(),
          @r#"
        [package]
        edition = "2024"
        name = "genotype_example_package"
        version = "0.1.0"
        [dependencies]
        genotype_json_types = "0.1.0"
        serde = { version = "1", features = ["derive"] }
        "#
        );

        assert_debug_snapshot!(
          dist.files[2].path(),
          @r#"
        GtpTargetFilePath(
            GtpCwdRelativePath(
                "examples/dependencies/dist/rs/src/lib.rs",
            ),
        )
        "#
        );
        assert_snapshot!(
          dist.files[2].source_code(),
          @"
        pub(crate) mod prompt;
        pub use prompt::*;
        "
        );

        assert_ron_snapshot!(
          dist.files[3].path(),
          @r#""examples/dependencies/dist/rs/src/prompt.rs""#
        );
        assert_snapshot!(
          dist.files[3].source_code(),
          @"
        use genotype_json_types::JsonAny;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Prompt {
            pub content: String,
            pub output: JsonAny,
        }
        "
        );
    }

    #[test]
    fn test_render_uses_global_version_by_default() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config_mut().version = Some("0.2.0".parse().unwrap());

        let dist = compile(&project);
        let cargo = get_cargo_file(&dist);

        assert_snapshot!(
            cargo.source_code(),
            @r#"
        [package]
        edition = "2024"
        name = "basic"
        version = "0.2.0"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );
    }

    #[test]
    fn test_render_prefers_rs_manifest_version_over_global() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config_mut().version = Some("0.2.0".parse().unwrap());
        project.config_mut().rs.common.manifest = toml::from_str(
            r#"[package]
version = "0.3.0"
"#,
        )
        .unwrap();

        let dist = compile(&project);
        let cargo = get_cargo_file(&dist);

        assert_snapshot!(
            cargo.source_code(),
            @r#"
        [package]
        edition = "2024"
        name = "basic"
        version = "0.3.0"

        [dependencies]
        serde = { version = "1", features = ["derive"] }
        "#
        );
    }

    #[test]
    fn test_render_without_package_global() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        project.config_mut().package = false;

        let dist = compile(&project);

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/rs/author.rs",
          "examples/basic/dist/rs/book.rs",
          "examples/basic/dist/rs/mod.rs",
        ]
        "#);
    }

    #[test]
    fn test_render_without_package_target() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        let config = project.config_mut();
        config.package = true;
        config.rs.common.package = Some(false);

        let dist = compile(&project);

        assert_ron_snapshot!(
          dist.files.iter().map(|file| file.path().as_str()).collect::<Vec<_>>(),
          @r#"
        [
          "examples/basic/dist/rs/author.rs",
          "examples/basic/dist/rs/book.rs",
          "examples/basic/dist/rs/mod.rs",
        ]
        "#);
    }

    #[test]
    fn test_dist_includes_module_errors() {
        let mut project =
            GtpRuntimeSystem::new_and_load_all_modules(&"./examples/basic".into(), None).unwrap();
        let path: GtpModulePath = "examples/basic/src/broken.type".into();
        let source = GtpModuleSource::Entry { path: path.clone() };
        project.modules_mut().insert(
            path.clone(),
            GtpModule::Error(
                source,
                GtpModuleError::Init {
                    path,
                    message: "synthetic parse failure".into(),
                },
            ),
        );

        let dist = compile(&project);

        assert!(dist.files.iter().any(|file| matches!(
            file,
            GtlDistFile::Error(error)
                if error.path.as_str() == "examples/basic/dist/rs/src/broken.rs"
                    && error.message.contains("Failed to convert")
        )));
    }

    fn modules(project: &GtProject) -> GtlProjectModules<RsProjectModule> {
        let compiler = RsCompiler::new(project);
        let mut lang_project = GtlProject::<RsProjectModule>::new(compiler.config());
        lang_project.convert(&project.modules());
        lang_project.resolve().unwrap();
        lang_project.modules
    }

    fn compile(project: &GtProject) -> GtlDist {
        RsCompiler::new(project).compile().unwrap().unwrap()
    }

    fn get_cargo_file(dist: &GtlDist) -> &GtlDistFile {
        dist.files
            .iter()
            .find(|file| file.path().as_str().contains("Cargo.toml"))
            .unwrap()
    }
}

// endregion
