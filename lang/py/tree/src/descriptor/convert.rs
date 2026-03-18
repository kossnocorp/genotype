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
    use insta::assert_ron_snapshot;

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
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
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
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut PYConvertContext::default()),
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
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut context),
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
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int32((0, 0).into()).into(),
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
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut PYConvertContext::default()),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_ron_snapshot!(
            GTDescriptor::Reference(
                GTReference {
                    span: (0, 0).into(),
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                        "module".into(),
                        "Name".into()
                    )),
                    identifier: GTIdentifier::new((0, 0).into(), "Name".into())
                }
                .into()
            )
            .convert(&mut PYConvertContext::default()),
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
            GTDescriptor::Tuple(GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut PYConvertContext::default()),
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
            GTDescriptor::Union(GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut PYConvertContext::default()),
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
            GTDescriptor::Branded(GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            })
            .convert(&mut context),
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
