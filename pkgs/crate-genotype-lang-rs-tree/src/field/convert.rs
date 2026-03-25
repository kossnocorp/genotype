use crate::prelude::internal::*;

impl RsConvert<RsField> for GtProperty {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsField> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;

        // Allows for renaming fields
        let mut attributes = context.drain_field_attributes();

        context.enter_parent(RsContextParent::Field(name.clone()));

        let descriptor = self.descriptor.convert(context)?;
        let descriptor = if self.required {
            descriptor
        } else {
            attributes.push(RsAttribute(
                r#"serde(default, skip_serializing_if = "Option::is_none")"#.into(),
            ));
            RsOption::new(descriptor).into()
        };

        let field = RsField {
            doc,
            attributes,
            name,
            descriptor,
        };

        context.exit_parent();
        Ok(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsField(
          doc: None,
          attributes: [
            RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
          ],
          name: RsFieldName("name"),
          descriptor: Option(RsOption(
            descriptor: Primitive(String),
          )),
        )
        "#
        );
    }

    #[test]
    fn test_convert_rename_attribute() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "helloWorld".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsField(
          doc: None,
          attributes: [
            RsAttribute("serde(rename = \"helloWorld\")"),
            RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
          ],
          name: RsFieldName("hello_world"),
          descriptor: Option(RsOption(
            descriptor: Primitive(String),
          )),
        )
        "#
        );
    }
}
