use crate::prelude::internal::*;

impl PYConvert<PYDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut PYConvertContext) -> PYDescriptor {
        match self {
            GTDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GTDescriptor::Array(array) => array.convert(context).into(),

            GTDescriptor::InlineImport(import) => {
                let reference = import.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Literal(literal) => literal.convert(context).into(),

            GTDescriptor::Object(object) => context.hoist(|context| object.convert(context)).into(),

            GTDescriptor::Primitive(primitive) => primitive.convert(context).into(),

            GTDescriptor::Record(record) => record.convert(context).into(),

            GTDescriptor::Reference(name) => {
                let reference = name.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Tuple(tuple) => tuple.convert(context).into(),

            GTDescriptor::Union(union) => union.convert(context).into(),

            GTDescriptor::Any(any) => any.convert(context).into(),

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
        let mut context = PYConvertContext::default();
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
            @r#"
        Reference(PYReference(
          identifier: PYIdentifier("Name"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(PYAlias(
            doc: None,
            name: PYIdentifier("Name"),
            descriptor: Primitive(Boolean),
            references: [],
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
        List(PYList(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::inline_import("./path/to/module", "Name")),
                &mut context
            ),
            @r#"
        Reference(PYReference(
          identifier: PYIdentifier("Name"),
          forward: false,
        ))
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Path(PYPath(".path.to.module")), PYIdentifier("Name")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_object() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTDescriptor::Object(GTObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: Gt::primitive_i32().into(),
                        required: false,
                    }
                ],
            })
            .convert(&mut context),
            @r#"
        Reference(PYReference(
          identifier: PYIdentifier("Person"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Class(PYClass(
            doc: None,
            name: PYIdentifier("Person"),
            extensions: [],
            properties: [
              PYProperty(
                doc: None,
                name: PYKey("name"),
                descriptor: Primitive(String),
                required: true,
              ),
              PYProperty(
                doc: None,
                name: PYKey("age"),
                descriptor: Primitive(Int),
                required: false,
              ),
            ],
            references: [],
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_ron_snapshot!(
            GTDescriptor::Primitive(Gt::primitive_boolean())
                .convert(&mut PYConvertContext::default()),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::reference("Name"))),
            @r#"
        Reference(PYReference(
          identifier: PYIdentifier("Name"),
          forward: true,
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
        Tuple(PYTuple(
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
        Union(PYUnion(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
          discriminator: None,
        ))
        "
        );
    }

    #[test]
    fn test_convert_branded() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::branded(
                    "UserId",
                    Gt::primitive_string()
                )),
                &mut context
            ),
            @r#"
        Reference(PYReference(
          identifier: PYIdentifier("UserId"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Newtype(PYNewtype(
            doc: None,
            name: PYIdentifier("UserId"),
            primitive: String,
          )),
        ]
        "#
        );
    }
}
