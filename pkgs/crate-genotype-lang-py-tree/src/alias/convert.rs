use crate::prelude::internal::*;

struct LegacyAliasSelfReferenceVisitor {
    name: PyIdentifier,
}

impl PyVisitor for LegacyAliasSelfReferenceVisitor {}

impl PyVisitorMut for LegacyAliasSelfReferenceVisitor {
    fn visit_reference_mut(&mut self, reference: &mut PyReference) {
        if reference.identifier == self.name {
            reference.forward = true;
        }
    }
}

impl PyConvert<PyDefinition> for GtAlias {
    fn convert(&self, context: &mut PyConvertContext) -> PyDefinition {
        let doc = self.doc.as_ref().map(|doc| doc.convert(context));

        let name = self.name.convert(context);
        context.push_defined(&name);

        match &self.descriptor {
            GtDescriptor::Object(object) => {
                context.provide_doc(doc);
                PyDefinition::Class(object.convert(context))
            }

            GtDescriptor::Branded(branded) => {
                context.provide_doc(doc);
                PyDefinition::Newtype(branded.convert(context))
            }

            _ => {
                context.create_references_scope();

                let mut descriptor = self.descriptor.convert(context);

                for attribute in self.attributes.iter() {
                    if let PyDescriptor::Union(union) = &mut descriptor
                        && let Some(assignment) = attribute.get_assigned("discriminator")
                        && let GtAttributeValue::Literal(literal) = &assignment.value
                        && let GtLiteralValue::String(value) = &literal.value
                    {
                        union.discriminator = value.clone().into();
                        // [TODO] Resolve right now is a mess, instead of resolving in
                        // convert functions, it should be resolved in the end or by
                        // the parent.
                        union.clone().resolve(context);
                    }
                }

                if context.is_version(PyVersion::Legacy) {
                    let mut visitor = LegacyAliasSelfReferenceVisitor { name: name.clone() };
                    descriptor.traverse_mut(&mut visitor);
                }

                let references = context.pop_references_scope();

                PyDefinition::Alias(PyAlias {
                    doc,
                    name,
                    descriptor,
                    references,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            convert_node(
                 Gt::alias("Name", Gt::primitive_boolean())
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_class() {
        assert_ron_snapshot!(
            convert_node(
                Gt::alias(
                    "Book",
                    Gt::object(
                        "Book",
                        vec![
                            Gt::property("title", Gt::primitive_string()),
                            Gt::property("author", Gt::primitive_string()),
                        ],
                    ),
                )
            ),
            @r#"
        Class(PyClass(
          doc: None,
          name: PyIdentifier("Book"),
          extensions: [],
          properties: [
            PyProperty(
              doc: None,
              name: PyKey("title"),
              descriptor: Primitive(String),
              required: true,
            ),
            PyProperty(
              doc: None,
              name: PyKey("author"),
              descriptor: Primitive(String),
              required: true,
            ),
          ],
          references: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            convert_node(
                Gt::alias(
                    "UserId",
                    Gt::descriptor(Gt::branded("UserId", Gt::primitive_string())),
                )
            ),
            @r#"
        Newtype(PyNewtype(
          doc: None,
          name: PyIdentifier("UserId"),
          primitive: String,
        ))
        "#
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias(
                    "Book",
                    Gt::descriptor(Gt::union(vec_into![
                        Gt::object("BookObj", vec![Gt::property("author", Gt::primitive_string())]),
                        Gt::primitive_string(),
                    ])),
                ),
                &mut context,
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Book"),
          descriptor: Union(PyUnion(
            descriptors: [
              Reference(PyReference(
                identifier: PyIdentifier("BookObj"),
                forward: true,
              )),
              Primitive(String),
            ],
            discriminator: None,
          )),
          references: [
            PyIdentifier("BookObj"),
          ],
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
            name: PyIdentifier("BookObj"),
            extensions: [],
            properties: [
              PyProperty(
                doc: None,
                name: PyKey("author"),
                descriptor: Primitive(String),
                required: true,
              ),
            ],
            references: [],
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Pyt::convert_context_legacy();

        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias("Name", Gt::primitive_string()),
                &mut context,
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert_ron_snapshot!(
            context.imports(),
            @"[]"
        )
    }

    #[test]
    fn test_forward_alias() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias("Name", Gt::primitive_string()),
                &mut context,
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GtIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GtIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_forward_class() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias("Name", Gt::object("Name", vec![])),
                &mut context,
            ),
            @r#"
        Class(PyClass(
          doc: None,
          name: PyIdentifier("Name"),
          extensions: [],
          properties: [],
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GtIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GtIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_convert_discriminator() {
        assert_ron_snapshot!(
            convert_node(
                assign!(
                    Gt::alias("Message", Gt::descriptor(Gt::union(vec_into![
                        Gt::reference_anon("Reply"),
                        Gt::reference_anon("DM"),
                    ]))),
                    attributes = vec![attribute_node!(discriminator = "type")]
                )
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Message"),
          descriptor: Union(PyUnion(
            descriptors: [
              Reference(PyReference(
                identifier: PyIdentifier("Reply"),
                forward: true,
              )),
              Reference(PyReference(
                identifier: PyIdentifier("DM"),
                forward: true,
              )),
            ],
            discriminator: Some("type"),
          )),
          references: [
            PyIdentifier("Reply"),
            PyIdentifier("DM"),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_legacy_self_ref_alias_array() {
        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias("SelfRefArray", Gt::array(Gt::reference_anon("SelfRefArray"))),
                &mut Pyt::convert_context_legacy(),
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("SelfRefArray"),
          descriptor: List(PyList(
            descriptor: Reference(PyReference(
              identifier: PyIdentifier("SelfRefArray"),
              forward: true,
            )),
          )),
          references: [
            PyIdentifier("SelfRefArray"),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_legacy_self_ref_alias_tuple() {
        assert_ron_snapshot!(
            convert_node_with(
                Gt::alias(
                    "SelfRefTuple",
                    Gt::union(vec![
                        Gt::literal_null().into(),
                        Gt::tuple(vec![
                            Gt::primitive_string().into(),
                            Gt::array(Gt::reference_anon("SelfRefTuple")).into(),
                        ])
                        .into(),
                    ]),
                ),
                &mut Pyt::convert_context_legacy(),
            ),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("SelfRefTuple"),
          descriptor: Union(PyUnion(
            descriptors: [
              Literal(r#None),
              Tuple(PyTuple(
                descriptors: [
                  Primitive(String),
                  List(PyList(
                    descriptor: Reference(PyReference(
                      identifier: PyIdentifier("SelfRefTuple"),
                      forward: true,
                    )),
                  )),
                ],
              )),
            ],
            discriminator: None,
          )),
          references: [
            PyIdentifier("SelfRefTuple"),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_ron_snapshot!(
            convert_node(
                assign!(
                    Gt::alias("Name", Gt::primitive_boolean()),
                    doc = Gt::some_doc("Hello, world!")
                ),
            ),
            @r#"
        Alias(PyAlias(
          doc: Some(PyDoc("Hello, world!")),
          name: PyIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }
}
