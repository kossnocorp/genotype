use crate::prelude::internal::*;

impl RsConvert<RsStruct> for GtObject {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsStruct> {
        let name = match &self.name {
            GtObjectName::Named(identifier) => identifier.convert(context),
            GtObjectName::Alias(identifier, _) => identifier.convert(context),
        }?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        context.enter_parent(RsContextParent::Definition(name.clone()));

        let doc = context.consume_doc();

        // Collect regular and literal fields separately. Literal fields will be
        // converted to attributes and won't be actual fields in the struct.
        // Having them as fields results in extra struct, naming issues, and
        // ultimately verbose code for the end user without any benefit.
        let mut fields = vec![];
        let mut literal_fields = vec![];
        for property in &self.properties {
            match &property.descriptor {
                GtDescriptor::Literal(literal) => {
                    let name = RsNaming::render(property.name.1.as_ref());
                    let value = render_literal(literal);
                    literal_fields.push(format!("{name} = {value}"));
                }

                _ => fields.push(property.convert(context)?),
            }
        }

        // If object has extension, we need to set fields to unresolved as Rust has no inheritance
        // and we need to copy fields from the parent struct after all the modules are known.
        let fields = if !self.extensions.is_empty() {
            let references = self
                .extensions
                .iter()
                .map(|e| e.reference.convert(context))
                .collect::<Result<Vec<_>>>()?;
            RsStructFields::Unresolved(self.span, references, fields)
        } else {
            RsStructFields::Resolved(fields)
        };

        let mut attributes = vec![{
            // Use Litty derives instead of Serde if there are literal fields. It is a drop-in
            // replacement and behaves the same for regular fields, but also adds support for
            // literal fields.
            let derive_mode = if literal_fields.is_empty() {
                RsContextRenderDeriveSerdeMode::Serde
            } else {
                RsContextRenderDeriveSerdeMode::Litty
            };
            context
                .render_derive(RsContextRenderDeriveTypeMode::Struct, derive_mode)
                .into()
        }];

        // Add literal fields via Litty's literals attribute.
        if !literal_fields.is_empty() {
            attributes.push(RsAttribute(format!(
                "literals({})",
                literal_fields.join(", ")
            )));
            context.push_import(RsUse::new(RsDependencyIdent::Litty, "Literals".into()));
        }

        let r#struct = RsStruct {
            id,
            doc,
            attributes,
            name,
            fields,
        };

        if literal_fields.is_empty() {
            context.push_import(RsUse::new(RsDependencyIdent::Serde, "Deserialize".into()));
            context.push_import(RsUse::new(RsDependencyIdent::Serde, "Serialize".into()));
        }

        context.exit_parent();
        Ok(r#struct)
    }
}

impl RsConvert<RsStruct> for GtLiteral {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsStruct> {
        context.push_import(RsUse::new(RsDependencyIdent::Litty, "literal".into()));

        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(Some(self.clone().into()))
        };
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));

        let literal = render_literal(self);

        Ok(RsStruct {
            id,
            doc,
            attributes: vec![RsAttribute(format!("literal({literal})"))],
            name,
            fields: RsStructFields::Unit,
        })
    }
}

impl RsConvert<RsStruct> for GtBranded {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsStruct> {
        let doc = context.consume_doc();
        let name = self.name.convert(context)?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        let descriptor = self.primitive.convert(context)?.into();

        Ok(RsStruct {
            id,
            doc,
            attributes: vec![
                context
                    .render_derive(
                        RsContextRenderDeriveTypeMode::Struct,
                        RsContextRenderDeriveSerdeMode::Serde,
                    )
                    .into(),
            ],
            name,
            fields: RsStructFields::Newtype(vec![descriptor]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert_object() {
        assert_ron_snapshot!(
            GtObject {
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
                ]
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("Person"),
          fields: Resolved([
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("name"),
              descriptor: Primitive(String),
            ),
            RsField(
              doc: None,
              attributes: [
                RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
              ],
              name: RsFieldName("age"),
              descriptor: Option(RsOption(
                descriptor: Primitive(Int32),
              )),
            ),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_object_import() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("Person"),
          fields: Resolved([]),
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Serde,
            reference: Named([
              Name(RsIdentifier("Deserialize")),
            ]),
          ),
          RsUse(
            dependency: Serde,
            reference: Named([
              Name(RsIdentifier("Serialize")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_object_literal_fields() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtObject {
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
                        name: GtKey::new((0, 0).into(), "ok".into()),
                        descriptor: Gt::literal_boolean(true).into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "version".into()),
                        descriptor: Gt::literal_integer(1).into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "message".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                ],
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("literals(ok = true, version = 1)"),
          ],
          name: RsIdentifier("Person"),
          fields: Resolved([
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("message"),
              descriptor: Primitive(String),
            ),
          ]),
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Litty,
            reference: Named([
              Name(RsIdentifier("Literals")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_object_literal_fields_unresolved() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![(Gt::reference_id((2, 9)), Gt::definition_id("Model"))],
        );
        assert_ron_snapshot!(
            GtObject {
                span: (1, 8).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![GtExtension {
                    span: (0, 0).into(),
                    reference: GtReference {
                        span: (2, 9).into(),
                        doc: None,
                        attributes: vec![],
                        id: GtReferenceId("module".into(), (2, 9).into()),
                        identifier: GtIdentifier::new((0, 0).into(), "Model".into()),
                        arguments: vec![],
                    },
                }],
                properties: vec![
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "ok".into()),
                        descriptor: Gt::literal_boolean(true).into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                ],
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Literals)"),
            RsAttribute("literals(ok = true)"),
          ],
          name: RsIdentifier("Person"),
          fields: Unresolved(GtSpan(1, 8), [
            RsReference(
              id: GtReferenceId(GtModuleId("module"), GtSpan(2, 9)),
              identifier: RsIdentifier("Model"),
              definition_id: GtDefinitionId(GtModuleId("module"), "Model"),
            ),
          ], [
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("name"),
              descriptor: Primitive(String),
            ),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_object_doc() {
        let mut context = RsConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: Some(RsDoc("Hello, world!", false)),
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("Person"),
          fields: Resolved([]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_object_unresolved() {
        let mut resolve = RsConvertResolve::default();
        resolve.reference_definition_ids.insert(
            GtReferenceId("module".into(), (2, 9).into()),
            GtDefinitionId("module".into(), "Model".into()),
        );
        let mut context = Rst::convert_context_with_resolve(resolve);
        assert_ron_snapshot!(
            GtObject {
                span: (1, 8).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![GtExtension {
                    span: (0, 0).into(),
                    reference: GtReference {
                        span: (2, 9).into(),
                        doc: None,
                        attributes: vec![],
                        id: GtReferenceId("module".into(), (2, 9).into()),
                        identifier: GtIdentifier::new((0, 0).into(), "Model".into()),
                        arguments: vec![],
                    },
                }],
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
                        descriptor: Gt::primitive_isize().into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("Person"),
          fields: Unresolved(GtSpan(1, 8), [
            RsReference(
              id: GtReferenceId(GtModuleId("module"), GtSpan(2, 9)),
              identifier: RsIdentifier("Model"),
              definition_id: GtDefinitionId(GtModuleId("module"), "Model"),
            ),
          ], [
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("name"),
              descriptor: Primitive(String),
            ),
            RsField(
              doc: None,
              attributes: [
                RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
              ],
              name: RsFieldName("age"),
              descriptor: Option(RsOption(
                descriptor: Primitive(IntSize),
              )),
            ),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal() {
        assert_ron_snapshot!(
            Gt::literal_boolean(true)
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "True"),
          doc: None,
          attributes: [
            RsAttribute("literal(true)"),
          ],
          name: RsIdentifier("True"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_name_from_alias() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Version".into()));
        assert_ron_snapshot!(
            Gt::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RsAttribute("literal(1)"),
          ],
          name: RsIdentifier("Version"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_name_from_parents() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("User".into()));
        context.enter_parent(RsContextParent::Field("v".into()));
        assert_ron_snapshot!(
            Gt::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "UserV1"),
          doc: None,
          attributes: [
            RsAttribute("literal(1)"),
          ],
          name: RsIdentifier("UserV1"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_import() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            Gt::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "False"),
          doc: None,
          attributes: [
            RsAttribute("literal(false)"),
          ],
          name: RsIdentifier("False"),
          fields: Unit,
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Litty,
            reference: Named([
              Name(RsIdentifier("literal")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_literal_doc() {
        let mut context = RsConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            Gt::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "False"),
          doc: Some(RsDoc("Hello, world!", false)),
          attributes: [
            RsAttribute("literal(false)"),
          ],
          name: RsIdentifier("False"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_float() {
        assert_ron_snapshot!(
            Gt::literal_float(1.23456)
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Lit1_23456"),
          doc: None,
          attributes: [
            RsAttribute("literal(1.23456)"),
          ],
          name: RsIdentifier("Lit1_23456"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            Gt::literal_boolean(true)
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "True"),
          doc: None,
          attributes: [
            RsAttribute("literal(true)"),
          ],
          name: RsIdentifier("True"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_name_from_alias() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Version".into()));
        assert_ron_snapshot!(
            Gt::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RsAttribute("literal(1)"),
          ],
          name: RsIdentifier("Version"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_name_from_parents() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("User".into()));
        context.enter_parent(RsContextParent::Field("v".into()));
        assert_ron_snapshot!(
            Gt::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "UserV1"),
          doc: None,
          attributes: [
            RsAttribute("literal(1)"),
          ],
          name: RsIdentifier("UserV1"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_import() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            Gt::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "False"),
          doc: None,
          attributes: [
            RsAttribute("literal(false)"),
          ],
          name: RsIdentifier("False"),
          fields: Unit,
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Litty,
            reference: Named([
              Name(RsIdentifier("literal")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_branded_doc() {
        let mut context = RsConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            Gt::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "False"),
          doc: Some(RsDoc("Hello, world!", false)),
          attributes: [
            RsAttribute("literal(false)"),
          ],
          name: RsIdentifier("False"),
          fields: Unit,
        )
        "#
        );
    }
}
