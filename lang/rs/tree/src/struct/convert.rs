use crate::prelude::internal::*;

impl RSConvert<RSStruct> for GTObject {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
        }?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        context.enter_parent(RSContextParent::Definition(name.clone()));

        let doc = context.consume_doc();
        let fields = self
            .properties
            .iter()
            .map(|p| p.convert(context))
            .collect::<Result<Vec<_>>>()?;

        // If object has extension, we need to set fields to unresolved as Rust has no inheritance
        // and we need to copy fields from the parent struct after all the modules are known.
        let fields = if self.extensions.len() > 0 {
            let references = self
                .extensions
                .iter()
                .map(|e| e.reference.convert(context))
                .collect::<Result<Vec<_>>>()?;
            RSStructFields::Unresolved(self.span.clone(), references, fields)
        } else {
            RSStructFields::Resolved(fields)
        };

        let r#struct = RSStruct {
            id,
            doc,
            attributes: vec![
                context
                    .render_derive(RSContextRenderDeriveMode::Struct)
                    .into(),
            ],
            name,
            fields,
        };

        context.add_import(RSDependencyIdent::Serde, "Deserialize".into());
        context.add_import(RSDependencyIdent::Serde, "Serialize".into());

        context.exit_parent();
        Ok(r#struct)
    }
}

impl RSConvert<RSStruct> for GTLiteral {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        context.add_import(RSDependencyIdent::Litty, "literal".into());

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

        Ok(RSStruct {
            id,
            doc,
            attributes: vec![RSAttribute(format!("literal({literal})"))],
            name,
            fields: RSStructFields::Unit,
        })
    }
}

impl RSConvert<RSStruct> for GTBranded {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        let doc = context.consume_doc();
        let name = self.name.convert(context)?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        let descriptor = self.primitive.convert(context)?.into();

        Ok(RSStruct {
            id,
            doc,
            attributes: vec![
                context
                    .render_derive(RSContextRenderDeriveMode::Struct)
                    .into(),
            ],
            name,
            fields: RSStructFields::Newtype(vec![descriptor]),
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
            GTObject {
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
                        descriptor: GtFactory::primitive_string().into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GtFactory::primitive_i32().into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("Person"),
          fields: Resolved([
            RSField(
              doc: None,
              attributes: [],
              name: RSFieldName("name"),
              descriptor: Primitive(String),
            ),
            RSField(
              doc: None,
              attributes: [
                RSAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
              ],
              name: RSFieldName("age"),
              descriptor: Option(RSOption(
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
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("Person"),
          fields: Resolved([]),
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Serde, RSIdentifier("Deserialize")),
          (Serde, RSIdentifier("Serialize")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_object_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            GTObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Person"),
          doc: Some(RSDoc("Hello, world!", false)),
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("Person"),
          fields: Resolved([]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_object_unresolved() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTObject {
                span: (1, 8).into(),
                doc: None,
                attributes: vec![],
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![GTExtension {
                    span: (0, 0).into(),
                    reference: GTReference {
                        span: (2, 9).into(),
                        doc: None,
                        attributes: vec![],
                        id: GTReferenceId("module".into(), (2, 9).into()),
                        definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                            "module".into(),
                            "Model".into()
                        )),
                        identifier: GTIdentifier::new((0, 0).into(), "Model".into())
                    }
                    .into(),
                }],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GtFactory::primitive_string().into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GtFactory::primitive_isize().into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Person"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("Person"),
          fields: Unresolved(GTSpan(1, 8), [
            RSReference(
              id: GTReferenceId(GTModuleId("module"), GTSpan(2, 9)),
              identifier: RSIdentifier("Model"),
              definition_id: GTDefinitionId(GTModuleId("module"), "Model"),
            ),
          ], [
            RSField(
              doc: None,
              attributes: [],
              name: RSFieldName("name"),
              descriptor: Primitive(String),
            ),
            RSField(
              doc: None,
              attributes: [
                RSAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
              ],
              name: RSFieldName("age"),
              descriptor: Option(RSOption(
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
            GtFactory::literal_boolean(true)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "True"),
          doc: None,
          attributes: [
            RSAttribute("literal(true)"),
          ],
          name: RSIdentifier("True"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_name_from_alias() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_ron_snapshot!(
            GtFactory::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RSAttribute("literal(1)"),
          ],
          name: RSIdentifier("Version"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_name_from_parents() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_ron_snapshot!(
            GtFactory::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "UserV1"),
          doc: None,
          attributes: [
            RSAttribute("literal(1)"),
          ],
          name: RSIdentifier("UserV1"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtFactory::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "False"),
          doc: None,
          attributes: [
            RSAttribute("literal(false)"),
          ],
          name: RSIdentifier("False"),
          fields: Unit,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Litty, RSIdentifier("literal")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_literal_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            GtFactory::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "False"),
          doc: Some(RSDoc("Hello, world!", false)),
          attributes: [
            RSAttribute("literal(false)"),
          ],
          name: RSIdentifier("False"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_literal_float() {
        assert_ron_snapshot!(
            GtFactory::literal_float(1.23456)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Lit1_23456"),
          doc: None,
          attributes: [
            RSAttribute("literal(1.23456)"),
          ],
          name: RSIdentifier("Lit1_23456"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            GtFactory::literal_boolean(true)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "True"),
          doc: None,
          attributes: [
            RSAttribute("literal(true)"),
          ],
          name: RSIdentifier("True"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_name_from_alias() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_ron_snapshot!(
            GtFactory::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Version"),
          doc: None,
          attributes: [
            RSAttribute("literal(1)"),
          ],
          name: RSIdentifier("Version"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_name_from_parents() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_ron_snapshot!(
            GtFactory::literal_integer(1)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "UserV1"),
          doc: None,
          attributes: [
            RSAttribute("literal(1)"),
          ],
          name: RSIdentifier("UserV1"),
          fields: Unit,
        )
        "#
        );
    }

    #[test]
    fn test_convert_branded_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtFactory::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "False"),
          doc: None,
          attributes: [
            RSAttribute("literal(false)"),
          ],
          name: RSIdentifier("False"),
          fields: Unit,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Litty, RSIdentifier("literal")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_branded_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_ron_snapshot!(
            GtFactory::literal_boolean(false)
                .convert(&mut context)
                .unwrap(),
            @r#"
        RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "False"),
          doc: Some(RSDoc("Hello, world!", false)),
          attributes: [
            RSAttribute("literal(false)"),
          ],
          name: RSIdentifier("False"),
          fields: Unit,
        )
        "#
        );
    }
}
