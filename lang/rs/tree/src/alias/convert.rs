use crate::prelude::internal::*;

impl RSConvert<RSDefinition> for GTAlias {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSDefinition> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;
        context.push_defined(&name);
        context.enter_parent(RSContextParent::Alias(name.clone()));

        let definition = match &self.descriptor {
            GTDescriptor::Object(object) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Struct(object.convert(context)?)
            }

            GTDescriptor::Branded(branded) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Struct(branded.convert(context)?)
            }

            GTDescriptor::Union(union) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Enum(union.convert(context)?)
            }

            _ => {
                let descriptor = self.descriptor.convert(context)?;
                let alias = RSDefinition::Alias(RSAlias {
                    id: self.id.clone(),
                    doc,
                    name,
                    descriptor,
                });

                alias
            }
        };

        context.exit_parent();
        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Alias(RSAlias(
          id: GTDefinitionId(GTModuleId("module"), "Name"),
          doc: None,
          name: RSIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_struct() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Struct(RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "Book"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("Book"),
          fields: Resolved([
            RSField(
              doc: None,
              attributes: [],
              name: RSFieldName("title"),
              descriptor: Primitive(String),
            ),
            RSField(
              doc: None,
              attributes: [],
              name: RSFieldName("author"),
              descriptor: Primitive(String),
            ),
          ]),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "BookId".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "BookId".into()),
                descriptor: GTDescriptor::Branded(GTBranded {
                    span: (0, 0).into(),
                    id: GTDefinitionId("module".into(), "BookId".into()),
                    name: GTIdentifier((0, 0).into(), "BookId".into()),
                    primitive: GTPrimitive::Int32((0, 0).into())
                })
                .into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Struct(RSStruct(
          id: GTDefinitionId(GTModuleId("module"), "BookId"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RSIdentifier("BookId"),
          fields: Newtype([
            Primitive(Int32),
          ]),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GTObject {
                            span: (0, 0).into(),
                            name: GTObjectName::Named(GTIdentifier::new(
                                (0, 0).into(),
                                "BookObj".into()
                            )),
                            extensions: vec![],
                            properties: vec![GTProperty {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((0, 0).into(), "author".into()),
                                descriptor: GTPrimitive::String((0, 0).into()).into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GTPrimitive::String((0, 0).into()).into(),
                    ]
                })
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        Enum(RSEnum(
          id: GTDefinitionId(GTModuleId("module"), "Book"),
          doc: None,
          attributes: [
            RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RSAttribute("serde(untagged)"),
          ],
          name: RSIdentifier("Book"),
          variants: [
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("Obj"),
              descriptor: Descriptor(Reference(RSReference(
                id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
                identifier: RSIdentifier("BookObj"),
                definition_id: GTDefinitionId(GTModuleId("module"), "BookObj"),
              ))),
            ),
            RSEnumVariant(
              doc: None,
              attributes: [],
              name: RSIdentifier("String"),
              descriptor: Descriptor(Primitive(String)),
            ),
          ],
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Struct(RSStruct(
            id: GTDefinitionId(GTModuleId("module"), "BookObj"),
            doc: None,
            attributes: [
              RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            ],
            name: RSIdentifier("BookObj"),
            fields: Resolved([
              RSField(
                doc: None,
                attributes: [],
                name: RSFieldName("author"),
                descriptor: Primitive(String),
              ),
            ]),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Alias(RSAlias(
          id: GTDefinitionId(GTModuleId("module"), "Name"),
          doc: Some(RSDoc("Hello, world!", false)),
          name: RSIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }
}
