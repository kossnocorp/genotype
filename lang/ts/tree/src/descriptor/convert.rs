use crate::prelude::internal::*;

impl TSConvert<TSDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut TSConvertContext) -> TSDescriptor {
        match self {
            GTDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GTDescriptor::Array(array) => TSDescriptor::Array(Box::new(array.convert(context))),

            GTDescriptor::InlineImport(import) => {
                TSDescriptor::InlineImport(import.convert(context))
            }

            GTDescriptor::Literal(literal) => TSDescriptor::Literal(literal.convert(context)),

            GTDescriptor::Object(object) => {
                let descriptor = TSDescriptor::Object(object.convert(context));
                if object.extensions.is_empty() {
                    descriptor
                } else {
                    let mut descriptors: Vec<TSDescriptor> = vec![descriptor];
                    let extensions = object
                        .extensions
                        .iter()
                        .map(|extension| TSDescriptor::from(extension.reference.convert(context)))
                        .collect::<Vec<TSDescriptor>>();
                    descriptors.extend(extensions);
                    TSDescriptor::Intersection(TSIntersection { descriptors })
                }
            }

            GTDescriptor::Primitive(primitive) => {
                TSDescriptor::Primitive(primitive.convert(context))
            }

            GTDescriptor::Reference(name) => TSDescriptor::Reference(name.convert(context)),

            GTDescriptor::Tuple(tuple) => TSDescriptor::Tuple(tuple.convert(context)),

            GTDescriptor::Union(union) => TSDescriptor::Union(union.convert(context)),

            GTDescriptor::Record(record) => TSDescriptor::Record(Box::new(record.convert(context))),

            GTDescriptor::Any(any) => TSDescriptor::Any(any.convert(context)),

            GTDescriptor::Branded(branded) => {
                context.hoist(|context| branded.convert(context)).into()
            }
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
        let mut context = Default::default();
        assert_ron_snapshot!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }))
            .convert(&mut context),
            @r#"Reference(TSReference(TSIdentifier("Name")))"#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(TSAlias(
            doc: None,
            name: TSIdentifier("Name"),
            descriptor: Primitive(Boolean),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_array() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::array(Gt::primitive_boolean()))),
            @"
        Array(TSArray(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(
                Gt::inline_import("./path/to/module", "Name")
            )),
            @r#"
        InlineImport(TSInlineImport(
          path: TSPath("./path/to/module"),
          name: TSIdentifier("Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_object() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::object("Person", vec![
                Gt::property("name", Gt::primitive_string()),
                Gt::property_optional("age", Gt::primitive_i32())
            ]))),
            @r#"
        Object(TSObject(
          properties: [
            TSProperty(
              doc: None,
              name: TSKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            TSProperty(
              doc: None,
              name: TSKey("age"),
              descriptor: Union(TSUnion(
                descriptors: [
                  Primitive(Number),
                  Primitive(Undefined),
                ],
              )),
              required: false,
            ),
          ],
        ))
        "#
        );

        assert_ron_snapshot!(
            convert_node(Gt::descriptor(GTObject {
                extensions: vec![GTExtension {
                    span: (0, 0).into(),
                    reference: Gt::reference("Good").into()
                }],
                ..Gt::object("Book", vec![
                    Gt::property("title", Gt::primitive_string()),
                ])
            })),
            @r#"
        Intersection(TSIntersection(
          descriptors: [
            Object(TSObject(
              properties: [
                TSProperty(
                  doc: None,
                  name: TSKey("title"),
                  descriptor: Primitive(String),
                  required: true,
                ),
              ],
            )),
            Reference(TSReference(TSIdentifier("Good"))),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::primitive_boolean())),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::reference("Name"))),
            @r#"Reference(TSReference(TSIdentifier("Name")))"#
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::tuple(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ]))),
            @"
        Tuple(TSTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        ))
        "
        );
    }

    #[test]
    fn test_convert_union() {
        assert_ron_snapshot!(
            GTDescriptor::Union(GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    Gt::primitive_boolean().into(),
                    Gt::primitive_string().into(),
                ]
            })
            .convert(&mut Default::default()),
            @"
        Union(TSUnion(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        ))
        "
        );
    }

    #[test]
    fn test_convert_record() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(
                Gt::record(Gt::record_key_string(), Gt::primitive_string())
            )),
            @"
        Record(TSRecord(
          key: String,
          descriptor: Primitive(String),
        ))
        "
        );
    }

    #[test]
    fn test_convert_any() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::any())),
            @"Any(TSAny)"
        );
    }

    #[test]
    fn test_convert_branded() {
        let mut context = Default::default();
        assert_ron_snapshot!(
            convert_node_with(Gt::descriptor(
                Gt::branded("UserId", Gt::primitive_string())
            ), &mut context),
            @r#"Reference(TSReference(TSIdentifier("UserId")))"#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Branded(TSBranded(
            doc: None,
            name: TSIdentifier("UserId"),
            primitive: String,
          )),
        ]
        "#
        );
    }
}
