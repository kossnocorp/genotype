use crate::prelude::internal::*;

mod visitor;
pub use visitor::*;

mod sort;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TsConvertModule(pub TsModule);

impl TsConvertModule {
    pub fn convert(module: &GtModule, resolve: TsConvertResolve, config: &TsConfig) -> Self {
        let mut context = TsConvertContext::new(resolve, config);
        let mode = config.lang.mode.clone();

        for import in &module.imports {
            let import = import.convert(&mut context);
            context.push_import(import);
        }

        let mut definitions = vec![];

        for alias in &module.aliases {
            let definition = alias.convert(&mut context);

            definitions.push(definition);
            definitions.extend(context.drain_hoisted());
        }

        let definitions = if mode == TsMode::Zod {
            Self::sort_definitions(definitions)
        } else {
            definitions
        };

        let doc = module.doc.as_ref().map(|doc| {
            let mut doc = doc.convert(&mut context);
            doc.0 = "@file ".to_string() + &doc.0;
            doc
        });

        let imports = context.drain_imports();

        let mut module = TsModule {
            doc,
            imports,
            definitions,
        };

        let mut visitor = TsModuleConvertVisitor::new(&module);
        module.traverse_mut(&mut visitor);

        TsConvertModule(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let mut resolve = TsConvertResolve::new();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
            "module".into(),
        );

        assert_ron_snapshot!(
            TsConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: None,
                    imports: vec![
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
                            reference: GtImportReference::Glob((0, 0).into())
                        },
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
                            reference: GtImportReference::Names(
                                (0, 0).into(),
                                vec![
                                    GtImportName::Name(
                                        (0, 0).into(),
                                        GtIdentifier::new((0, 0).into(), "Name".into())
                                    ),
                                    GtImportName::Alias(
                                        (0, 0).into(),
                                        GtIdentifier::new((0, 0).into(), "Name".into()),
                                        GtIdentifier::new((0, 0).into(), "Alias".into())
                                    )
                                ]
                            )
                        }
                    ],
                    aliases: vec![
                        Gt::alias("User", Gt::object("User", vec![
                                Gt::property("name", Gt::primitive_string()),
                                Gt::property_optional("age", Gt::primitive_i32()),
                            ])),
                        Gt::alias("Order", Gt::object("Order", vec![
                                Gt::property("book", GtDescriptor::Alias(Box::new(
                                    Gt::alias("Book", Gt::object("Book", vec![
                                        Gt::property("title", Gt::primitive_string()),
                                        Gt::property("author", Gt::reference_anon("Author")),
                                    ])),
                                ))),
                            ])),
                        Gt::alias("Name", Gt::primitive_string()),
                    ],
                },
                resolve,
                &Default::default(),
            ),
            @r#"
        TsConvertModule(TsModule(
          doc: None,
          imports: [
            TsImport(
              dependency: Local(TsPath("./path/to/module")),
              reference: Glob("module"),
            ),
            TsImport(
              dependency: Local(TsPath("./path/to/module")),
              reference: Named([
                Name(TsIdentifier("Name")),
                Alias(TsIdentifier("Name"), TsIdentifier("Alias")),
              ]),
            ),
          ],
          definitions: [
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("User"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("name"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                TsProperty(
                  doc: None,
                  name: TsKey("age"),
                  descriptor: Union(TsUnion(
                    descriptors: [
                      Primitive(Number),
                      Primitive(Undefined),
                    ],
                  )),
                  required: false,
                ),
              ],
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Order"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("book"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Book"),
                    arguments: [],
                    rel: Forward,
                  )),
                  required: true,
                ),
              ],
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Book"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("title"),
                  descriptor: Primitive(String),
                  required: true,
                ),
                TsProperty(
                  doc: None,
                  name: TsKey("author"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Author"),
                    arguments: [],
                    rel: Regular,
                  )),
                  required: true,
                ),
              ],
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Name"),
              generics: [],
              descriptor: Primitive(String),
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            TsConvertModule::convert(
                &GtModule {
                    id: "module".into(),
                    doc: Gt::some_doc("Hello, world!"),
                    imports: vec![],
                    aliases: vec![],
                },
                TsConvertResolve::new(),
                &Default::default(),
            ),
            @r#"
        TsConvertModule(TsModule(
          doc: Some(TsDoc("@file Hello, world!")),
          imports: [],
          definitions: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_preserve_inline_imports() {
        assert_ron_snapshot!(
            TsConvertModule::convert(
                &Gt::module(
                    vec![Gt::import(
                        "./schemas",
                         Gt::import_reference_name("Base"),
                    )],
                    vec_into![
                        Gt::alias(
                            "User",
                            Gt::inline_import_anon("./schemas", "UserSchema")
                        ),
                        Gt::alias(
                            "UserId",
                            Gt::inline_import_anon("./ids", "Id")
                        ),
                    ]
                ),
                Default::default(),
                &Default::default()
            ),
            @r#"
        TsConvertModule(TsModule(
          doc: None,
          imports: [
            TsImport(
              dependency: Local(TsPath("./schemas")),
              reference: Named([
                Name(TsIdentifier("Base")),
              ]),
            ),
          ],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("User"),
              generics: [],
              descriptor: InlineImport(TsInlineImport(
                path: TsPath("./schemas"),
                name: TsIdentifier("UserSchema"),
                arguments: [],
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("UserId"),
              generics: [],
              descriptor: InlineImport(TsInlineImport(
                path: TsPath("./ids"),
                name: TsIdentifier("Id"),
                arguments: [],
              )),
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_zod_bubble_ups_inline_imports() {
        let mut config = TsConfig::default();
        config.lang.mode = TsMode::Zod;
        assert_ron_snapshot!(
            TsConvertModule::convert(
                &Gt::module(
                    vec![Gt::import(
                        "./schemas",
                         Gt::import_reference_name("Base"),
                    )],
                    vec_into![
                        Gt::alias(
                            "User",
                            Gt::inline_import_anon("./schemas", "UserSchema")
                        ),
                        Gt::alias(
                            "UserId",
                            Gt::inline_import_anon("./ids", "Id")
                        ),
                    ]
                ),
                TsConvertResolve::new(),
                &config
            ),
            @r#"
        TsConvertModule(TsModule(
          doc: None,
          imports: [
            TsImport(
              dependency: Local(TsPath("./schemas")),
              reference: Named([
                Name(TsIdentifier("Base")),
                Name(TsIdentifier("UserSchema")),
              ]),
            ),
            TsImport(
              dependency: Local(TsPath("./ids")),
              reference: Named([
                Name(TsIdentifier("Id")),
              ]),
            ),
          ],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("User"),
              generics: [],
              descriptor: InlineImport(TsInlineImport(
                path: TsPath("./schemas"),
                name: TsIdentifier("UserSchema"),
                arguments: [],
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("UserId"),
              generics: [],
              descriptor: InlineImport(TsInlineImport(
                path: TsPath("./ids"),
                name: TsIdentifier("Id"),
                arguments: [],
              )),
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_sorts_definitions_and_marks_references() {
        let module = Gt::module(
            vec![],
            vec![
                // Qwe -> Asd
                Gt::alias(
                    "Qwe",
                    Gt::object("Qwe", vec![Gt::property("asd", Gt::reference_anon("Asd"))]),
                ),
                // Asd -> Zxc
                Gt::alias(
                    "Asd",
                    Gt::object("Asd", vec![Gt::property("zxc", Gt::reference_anon("Zxc"))]),
                ),
                // Zxc -> <nothing>
                Gt::alias("Zxc", Gt::primitive_string()),
            ],
        );

        let mut config = TsConfig::default();
        config.lang.mode = TsMode::Zod;
        let converted = TsConvertModule::convert(&module, Default::default(), &config).0;

        let positions = converted
            .definitions
            .iter()
            .enumerate()
            .map(|(idx, definition)| (idx, definition.name()))
            .collect::<Vec<_>>();

        assert_ron_snapshot!(
            positions,
            @r#"
        [
          (0, TsIdentifier("Zxc")),
          (1, TsIdentifier("Asd")),
          (2, TsIdentifier("Qwe")),
        ]
        "#
        );

        assert_ron_snapshot!(converted, @r#"
        TsModule(
          doc: None,
          imports: [],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Zxc"),
              generics: [],
              descriptor: Primitive(String),
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Asd"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("zxc"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Zxc"),
                    arguments: [],
                    rel: Regular,
                  )),
                  required: true,
                ),
              ],
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Qwe"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("asd"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Asd"),
                    arguments: [],
                    rel: Regular,
                  )),
                  required: true,
                ),
              ],
            )),
          ],
        )
        "#);
    }
    #[test]
    fn test_convert_sorts_cyclic_definitions_and_marks_references() {
        let module = Gt::module(
            vec![],
            vec![
                // Bar -> Foo
                Gt::alias(
                    "Bar",
                    Gt::object("Bar", vec![Gt::property("foo", Gt::reference_anon("Foo"))]),
                ),
                // Baz -> Foo
                Gt::alias("Baz", Gt::reference_anon("Foo")),
                // Foo -> Bar
                Gt::alias(
                    "Foo",
                    Gt::object("Foo", vec![Gt::property("bar", Gt::reference_anon("Bar"))]),
                ),
            ],
        );

        let mut config = TsConfig::default();
        config.lang.mode = TsMode::Zod;
        let converted = TsConvertModule::convert(&module, Default::default(), &config).0;

        let positions = converted
            .definitions
            .iter()
            .enumerate()
            .map(|(idx, definition)| (idx, definition.name()))
            .collect::<Vec<_>>();

        assert_ron_snapshot!(
            positions,
            @r#"
        [
          (0, TsIdentifier("Foo")),
          (1, TsIdentifier("Bar")),
          (2, TsIdentifier("Baz")),
        ]
        "#
        );

        assert_ron_snapshot!(converted, @r#"
        TsModule(
          doc: None,
          imports: [],
          definitions: [
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Foo"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("bar"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Bar"),
                    arguments: [],
                    rel: Forward,
                  )),
                  required: true,
                ),
              ],
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Bar"),
              generics: [],
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("foo"),
                  descriptor: Reference(TsReference(
                    identifier: TsIdentifier("Foo"),
                    arguments: [],
                    rel: Regular,
                  )),
                  required: true,
                ),
              ],
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Baz"),
              generics: [],
              descriptor: Reference(TsReference(
                identifier: TsIdentifier("Foo"),
                arguments: [],
                rel: Regular,
              )),
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_sorts_cyclic_group_to_reduce_forward_references() {
        let module = Gt::module(
            vec![],
            vec![
                // JsonAny -> JsonArray | JsonObject | JsonProperty
                Gt::alias(
                    "JsonAny",
                    Gt::union(vec![
                        Gt::reference_anon("JsonArray").into(),
                        Gt::reference_anon("JsonObject").into(),
                        Gt::reference_anon("JsonProperty").into(),
                    ]),
                ),
                // JsonArray -> JsonAny
                Gt::alias(
                    "JsonArray",
                    Gt::object(
                        "JsonArray",
                        vec![Gt::property("descriptor", Gt::reference_anon("JsonAny"))],
                    ),
                ),
                // JsonObject -> JsonProperty
                Gt::alias(
                    "JsonObject",
                    Gt::object(
                        "JsonObject",
                        vec![Gt::property(
                            "properties",
                            Gt::array(Gt::reference_anon("JsonProperty")),
                        )],
                    ),
                ),
                // JsonProperty -> JsonAny
                Gt::alias(
                    "JsonProperty",
                    Gt::object(
                        "JsonProperty",
                        vec![Gt::property("descriptor", Gt::reference_anon("JsonAny"))],
                    ),
                ),
            ],
        );

        let mut config = TsConfig::default();
        config.lang.mode = TsMode::Zod;
        let converted = TsConvertModule::convert(&module, Default::default(), &config).0;

        let positions = converted
            .definitions
            .iter()
            .enumerate()
            .map(|(idx, definition)| (idx, definition.name()))
            .collect::<Vec<_>>();

        assert_ron_snapshot!(
            positions,
            @r#"
        [
          (0, TsIdentifier("JsonProperty")),
          (1, TsIdentifier("JsonObject")),
          (2, TsIdentifier("JsonArray")),
          (3, TsIdentifier("JsonAny")),
        ]
        "#
        );
    }
}
