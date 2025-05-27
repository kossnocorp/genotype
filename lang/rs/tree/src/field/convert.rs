use crate::prelude::internal::*;

impl RSConvert<RSField> for GTProperty {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSField> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;

        // Allows for renaming fields
        let mut attributes = context.drain_field_attributes();

        context.enter_parent(RSContextParent::Field(name.clone()));

        let descriptor = self.descriptor.convert(context)?;
        let descriptor = if self.required {
            descriptor
        } else {
            attributes.push(RSAttribute(
                r#"serde(default, skip_serializing_if = "Option::is_none")"#.into(),
            ));
            RSOption::new(descriptor).into()
        };

        let field = RSField {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context)
            .unwrap(),
            RSField {
                doc: None,
                attributes: vec![
                    r#"serde(default, skip_serializing_if = "Option::is_none")"#.into()
                ],
                name: "name".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
    }

    #[test]
    fn test_convert_rename_attribute() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "helloWorld".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context)
            .unwrap(),
            RSField {
                doc: None,
                attributes: vec![
                    r#"serde(rename = "helloWorld")"#.into(),
                    r#"serde(default, skip_serializing_if = "Option::is_none")"#.into()
                ],
                name: "hello_world".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
    }
}
