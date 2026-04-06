use crate::prelude::internal::*;

impl TsConvert<TsDefinition> for GtAlias {
    fn convert(&self, context: &mut TsConvertContext) -> TsDefinition {
        let doc = self.doc.as_ref().map(|d| d.convert(context));
        let name = self.name.convert(context);

        match &self.descriptor {
            GtDescriptor::Branded(branded) => {
                context.provide_doc(doc);
                TsDefinition::Branded(branded.convert(context))
            }

            GtDescriptor::Object(object) => TsDefinition::Interface(TsInterface {
                doc,
                name,
                extensions: object
                    .extensions
                    .iter()
                    .map(|e| e.convert(context))
                    .collect(),
                properties: object
                    .properties
                    .iter()
                    .map(|p| p.convert(context))
                    .collect(),
            }),

            _ => TsDefinition::Alias(TsAlias {
                doc,
                name,
                descriptor: self.descriptor.convert(context),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            convert_node(Gt::alias("Name", Gt::primitive_boolean())),
            @r#"
        Alias(TsAlias(
          doc: None,
          name: TsIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_ron_snapshot!(
            convert_node(Gt::alias("Book", Gt::object("Book", vec![
                Gt::property("title", Gt::primitive_string()),
                Gt::property("author", Gt::primitive_string())
            ]))),
            @r#"
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
              descriptor: Primitive(String),
              required: true,
            ),
          ],
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            convert_node(
                Gt::alias(
                    "BookId",
                    Gt::branded("BookId", Gt::primitive_string())
                )
            ),
            @r#"
        Branded(TsBranded(
          doc: None,
          name: TsIdentifier("BookId"),
          primitive: String,
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_extensions() {
        assert_ron_snapshot!(
            convert_node(Gt::alias("Book", GtObject {
                extensions: vec![GtExtension {
                    span: (0, 0).into(),
                    reference: Gt::reference("Good").into()
                }],
                ..Gt::object("Book", vec![
                    Gt::property("author", Gt::primitive_string())
                ])
            })),
            @r#"
        Interface(TsInterface(
          doc: None,
          name: TsIdentifier("Book"),
          extensions: [
            TsExtension(
              reference: TsReference(
                identifier: TsIdentifier("Good"),
                rel: Regular,
              ),
            ),
          ],
          properties: [
            TsProperty(
              doc: None,
              name: TsKey("author"),
              descriptor: Primitive(String),
              required: true,
            ),
          ],
        ))
        "#,
        );

        assert_ron_snapshot!(
            convert_node(Gt::alias("Book", Gt::union(
                vec_into![
                    GtObject {
                        name: GtObjectName::Alias(
                            GtIdentifier::new((0, 0).into(), "BookAuthorObj".into()),
                            GtObjectNameParent::Alias(Gt::identifier("BookAuthor"))
                        ),
                        extensions: vec![GtExtension {
                            span: (0, 0).into(),
                            reference: Gt::reference("Good").into()
                        }],
                        ..Gt::object("Book", vec![
                            Gt::property("author", Gt::primitive_string())
                        ])
                    },
                    Gt::primitive_string()
                ]
            ))),
            @r#"
        Alias(TsAlias(
          doc: None,
          name: TsIdentifier("Book"),
          descriptor: Union(TsUnion(
            descriptors: [
              Intersection(TsIntersection(
                descriptors: [
                  Object(TsObject(
                    properties: [
                      TsProperty(
                        doc: None,
                        name: TsKey("author"),
                        descriptor: Primitive(String),
                        required: true,
                      ),
                    ],
                  )),
                  Reference(TsReference(
                    identifier: TsIdentifier("Good"),
                    rel: Regular,
                  )),
                ],
              )),
              Primitive(String),
            ],
          )),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_interface() {
        assert_ron_snapshot!(
            convert_node(assign!(
                Gt::alias("Book", Gt::object("Book", vec![])),
                doc = Gt::some_doc("Hello, world!")
            )),
            @r#"
        Interface(TsInterface(
          doc: Some(TsDoc("Hello, world!")),
          name: TsIdentifier("Book"),
          extensions: [],
          properties: [],
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_ron_snapshot!(
            convert_node(assign!(
                Gt::alias("Name", Gt::primitive_boolean()),
                doc = Gt::some_doc("Hello, world!")
            )),
            @r#"
        Alias(TsAlias(
          doc: Some(TsDoc("Hello, world!")),
          name: TsIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_branded() {
        assert_ron_snapshot!(
            convert_node(assign!(
                Gt::alias("BookId", Gt::branded("BookId", Gt::primitive_string())),
                doc = Gt::some_doc("Hello, world!")
            )),
            @r#"
        Branded(TsBranded(
          doc: Some(TsDoc("Hello, world!")),
          name: TsIdentifier("BookId"),
          primitive: String,
        ))
        "#,
        );
    }
}
