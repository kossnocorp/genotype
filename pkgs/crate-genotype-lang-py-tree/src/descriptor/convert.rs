use crate::prelude::internal::*;

impl PyConvert<PyDescriptor> for GtDescriptor {
    fn convert(&self, context: &mut PyConvertContext) -> PyDescriptor {
        match self {
            GtDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GtDescriptor::Array(array) => array.convert(context).into(),

            GtDescriptor::InlineImport(import) => {
                let reference = import.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GtDescriptor::Literal(literal) => literal.convert(context).into(),

            GtDescriptor::Object(object) => context.hoist(|context| object.convert(context)).into(),

            GtDescriptor::Primitive(primitive) => primitive.convert(context).into(),

            GtDescriptor::Record(record) => record.convert(context).into(),

            GtDescriptor::Reference(name) => {
                let reference = name.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GtDescriptor::Tuple(tuple) => tuple.convert(context).into(),

            GtDescriptor::Union(union) => union.convert(context).into(),

            GtDescriptor::Any(any) => any.convert(context).into(),

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
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            GtDescriptor::Alias(Box::new(GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }))
            .convert(&mut context),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("Name"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(PyAlias(
            doc: None,
            name: PyIdentifier("Name"),
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
        List(PyList(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::inline_import("./path/to/module", "Name")),
                &mut context
            ),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("Name"),
          forward: false,
        ))
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Path(PyPath(".path.to.module")),
            reference: Named([
              Name(PyIdentifier("Name")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_object() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            GtDescriptor::Object(GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "age".into()),
                        descriptor: Gt::primitive_i32().into(),
                        required: false,
                    }
                ],
            })
            .convert(&mut context),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("Person"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Class(PyClass(
            doc: None,
            name: PyIdentifier("Person"),
            extensions: [],
            properties: [
              PyProperty(
                doc: None,
                name: PyKey("name"),
                descriptor: Primitive(String),
                required: true,
              ),
              PyProperty(
                doc: None,
                name: PyKey("age"),
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
            GtDescriptor::Primitive(Gt::primitive_boolean())
                .convert(&mut PyConvertContext::default()),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::reference("Name"))),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("Name"),
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
        Tuple(PyTuple(
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
        Union(PyUnion(
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
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::branded(
                    "UserId",
                    Gt::primitive_string()
                )),
                &mut context
            ),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("UserId"),
          forward: true,
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Newtype(PyNewtype(
            doc: None,
            name: PyIdentifier("UserId"),
            primitive: String,
          )),
        ]
        "#
        );
    }
}
