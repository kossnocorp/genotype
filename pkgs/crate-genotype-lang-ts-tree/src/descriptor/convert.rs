use crate::prelude::internal::*;

impl TsConvert<TsDescriptor> for GtDescriptor {
    fn convert(&self, context: &mut TsConvertContext) -> TsDescriptor {
        match self {
            GtDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GtDescriptor::Array(array) => TsDescriptor::Array(Box::new(array.convert(context))),

            GtDescriptor::InlineImport(import) => {
                TsDescriptor::InlineImport(import.convert(context))
            }

            GtDescriptor::Literal(literal) => TsDescriptor::Literal(literal.convert(context)),

            GtDescriptor::Object(object) => {
                let descriptor = TsDescriptor::Object(object.convert(context));
                if object.extensions.is_empty() {
                    descriptor
                } else {
                    let mut descriptors: Vec<TsDescriptor> = vec![descriptor];
                    let extensions = object
                        .extensions
                        .iter()
                        .map(|extension| TsDescriptor::from(extension.reference.convert(context)))
                        .collect::<Vec<TsDescriptor>>();
                    descriptors.extend(extensions);
                    TsDescriptor::Intersection(TsIntersection { descriptors })
                }
            }

            GtDescriptor::Primitive(primitive) => {
                TsDescriptor::Primitive(primitive.convert(context))
            }

            GtDescriptor::Reference(name) => TsDescriptor::Reference(name.convert(context)),

            GtDescriptor::Tuple(tuple) => TsDescriptor::Tuple(tuple.convert(context)),

            GtDescriptor::Union(union) => TsDescriptor::Union(union.convert(context)),

            GtDescriptor::Record(record) => TsDescriptor::Record(Box::new(record.convert(context))),

            GtDescriptor::Any(any) => TsDescriptor::Any(any.convert(context)),

            GtDescriptor::Branded(branded) => {
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
            convert_node_with(
                GtDescriptor::Alias(Box::new(Gt::alias("Name", Gt::primitive_boolean()))),
                &mut context,
            ),
            @r#"
        Reference(TsReference(
          identifier: TsIdentifier("Name"),
          rel: Regular,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(TsAlias(
            doc: None,
            name: TsIdentifier("Name"),
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
        Array(TsArray(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(
                Gt::inline_import_anon("./path/to/module", "Name")
            )),
            @r#"
        InlineImport(TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
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
        Object(TsObject(
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
        ))
        "#
        );

        assert_ron_snapshot!(
            convert_node(Gt::descriptor(GtObject {
                extensions: vec![GtExtension {
                    span: (0, 0).into(),
                    reference: Gt::reference_anon("Good")
                }],
                ..Gt::object("Book", vec![
                    Gt::property("title", Gt::primitive_string()),
                ])
            })),
            @r#"
        Intersection(TsIntersection(
          descriptors: [
            Object(TsObject(
              properties: [
                TsProperty(
                  doc: None,
                  name: TsKey("title"),
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
            convert_node(Gt::descriptor(Gt::reference_anon("Name"))),
            @r#"
        Reference(TsReference(
          identifier: TsIdentifier("Name"),
          rel: Regular,
        ))
        "#
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
        Tuple(TsTuple(
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
            convert_node(Gt::descriptor(Gt::union(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ]))),
            @"
        Union(TsUnion(
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
        Record(TsRecord(
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
            @"Any(TsAny)"
        );
    }

    #[test]
    fn test_convert_branded() {
        let mut context = Default::default();
        assert_ron_snapshot!(
            convert_node_with(Gt::descriptor(
                Gt::branded("UserId", Gt::primitive_string())
            ), &mut context),
            @r#"
        Reference(TsReference(
          identifier: TsIdentifier("UserId"),
          rel: Regular,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Branded(TsBranded(
            doc: None,
            name: TsIdentifier("UserId"),
            primitive: String,
          )),
        ]
        "#
        );
    }
}
