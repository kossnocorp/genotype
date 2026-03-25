use crate::prelude::internal::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TsConvertModule(pub TsModule);

impl TsConvertModule {
    pub fn convert(
        module: &GtModule,
        resolve: TsConvertResolve,
        dependencies_config: HashMap<String, String>,
    ) -> Self {
        let mut context = TsConvertContext::new(resolve, dependencies_config);

        let imports = module
            .imports
            .iter()
            .map(|import| import.convert(&mut context))
            .collect();

        let mut definitions = vec![];

        for alias in &module.aliases {
            let definition = alias.convert(&mut context);

            definitions.push(definition);
            definitions.extend(context.drain_hoisted());
        }

        let doc = module.doc.as_ref().map(|doc| {
            let mut doc = doc.convert(&mut context);
            doc.0 = "@file ".to_string() + &doc.0;
            doc
        });

        TsConvertModule(TsModule {
            doc,
            imports,
            definitions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut resolve = TsConvertResolve::new();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
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
                            path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                            reference: GtImportReference::Glob((0, 0).into())
                        },
                        GtImport {
                            span: (0, 0).into(),
                            path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
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
                        GtAlias {
                            id: GtDefinitionId("module".into(), "User".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "User".into()),
                            descriptor: Gt::object("User", vec![
                                Gt::property("name", Gt::primitive_string()),
                                Gt::property_optional("age", Gt::primitive_i32()),
                            ]).into(),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Order".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Order".into()),
                            descriptor: Gt::object("Order", vec![
                                Gt::property("book", GtDescriptor::Alias(Box::new(GtAlias {
                                    id: GtDefinitionId("module".into(), "Book".into()),
                                    span: (0, 0).into(),
                                    doc: None,
                                    attributes: vec![],
                                    name: GtIdentifier::new((0, 0).into(), "Book".into()),
                                    descriptor: Gt::object("Book", vec![
                                        Gt::property("title", Gt::primitive_string()),
                                        Gt::property("author", Gt::reference("Author")),
                                    ]).into(),
                                }))),
                            ]).into(),
                        },
                        GtAlias {
                            id: GtDefinitionId("module".into(), "Name".into()),
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtIdentifier::new((0, 0).into(), "Name".into()),
                            descriptor: Gt::primitive_string().into(),
                        },
                    ],
                },
                resolve,
                Default::default()
            ),
            @r#"
        TsConvertModule(TsModule(
          doc: None,
          imports: [
            TsImport(
              path: TsPath("./path/to/module"),
              reference: Glob("module"),
            ),
            TsImport(
              path: TsPath("./path/to/module"),
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
              extensions: [],
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("book"),
                  descriptor: Reference(TsReference(TsIdentifier("Book"))),
                  required: true,
                ),
              ],
            )),
            Interface(TsInterface(
              doc: None,
              name: TsIdentifier("Book"),
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
                  descriptor: Reference(TsReference(TsIdentifier("Author"))),
                  required: true,
                ),
              ],
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Name"),
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
                    doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                    imports: vec![],
                    aliases: vec![],
                },
                TsConvertResolve::new(),
                Default::default()
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
}
