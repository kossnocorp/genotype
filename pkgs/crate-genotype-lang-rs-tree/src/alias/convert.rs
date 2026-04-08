use crate::prelude::internal::*;

impl RsConvert<RsDefinition> for GtAlias {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsDefinition> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;
        context.push_defined(&name);
        context.enter_parent(RsContextParent::Alias(name.clone()));

        let definition = match &self.descriptor {
            GtDescriptor::Object(object) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RsDefinition::Struct(object.convert(context)?)
            }

            GtDescriptor::Branded(branded) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RsDefinition::Struct(branded.convert(context)?)
            }

            GtDescriptor::Union(union) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RsDefinition::Enum(union.convert(context)?)
            }

            _ => {
                let descriptor = self.descriptor.convert(context)?;
                

                RsDefinition::Alias(RsAlias {
                    id: self.id.clone(),
                    doc,
                    name,
                    descriptor,
                })
            }
        };

        context.exit_parent();
        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Alias(RsAlias(
          id: GtDefinitionId(GtModuleId("module"), "Name"),
          doc: None,
          name: RsIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_struct() {
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GtDescriptor::Object(GtObject {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    name: GtIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GtProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtKey::new((0, 0).into(), "title".into()),
                            descriptor: Gt::primitive_string().into(),
                            required: true,
                        },
                        GtProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtKey::new((0, 0).into(), "author".into()),
                            descriptor: Gt::primitive_string().into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Struct(RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "Book"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("Book"),
          fields: Resolved([
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("title"),
              descriptor: Primitive(String),
            ),
            RsField(
              doc: None,
              attributes: [],
              name: RsFieldName("author"),
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
            GtAlias {
                id: GtDefinitionId("module".into(), "BookId".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "BookId".into()),
                descriptor: Gt::descriptor(
                    Gt::branded("BookId", Gt::primitive_i32())
                )
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Struct(RsStruct(
          id: GtDefinitionId(GtModuleId("module"), "BookId"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
          ],
          name: RsIdentifier("BookId"),
          fields: Newtype([
            Primitive(Int32),
          ]),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: Gt::descriptor(Gt::union(vec_into![
                    Gt::object(
                        "BookObj",
                        vec![Gt::property("author", Gt::primitive_string())]
                    ),
                    Gt::primitive_string()
                ]))
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        Enum(RsEnum(
          id: GtDefinitionId(GtModuleId("module"), "Book"),
          doc: None,
          attributes: [
            RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            RsAttribute("serde(untagged)"),
          ],
          name: RsIdentifier("Book"),
          variants: [
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("Obj"),
              descriptor: Some(Descriptor(Reference(RsReference(
                id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
                identifier: RsIdentifier("BookObj"),
                definition_id: GtDefinitionId(GtModuleId("module"), "BookObj"),
              )))),
            ),
            RsEnumVariant(
              doc: None,
              attributes: [],
              name: RsIdentifier("String"),
              descriptor: Some(Descriptor(Primitive(String))),
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
          Struct(RsStruct(
            id: GtDefinitionId(GtModuleId("module"), "BookObj"),
            doc: None,
            attributes: [
              RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            ],
            name: RsIdentifier("BookObj"),
            fields: Resolved([
              RsField(
                doc: None,
                attributes: [],
                name: RsFieldName("author"),
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
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        Alias(RsAlias(
          id: GtDefinitionId(GtModuleId("module"), "Name"),
          doc: Some(RsDoc("Hello, world!", false)),
          name: RsIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }
}
