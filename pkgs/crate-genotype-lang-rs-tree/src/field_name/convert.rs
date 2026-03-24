use crate::prelude::internal::*;
use heck::ToSnakeCase;

impl RSConvert<RSFieldName> for GTKey {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSFieldName> {
        let name = self.1.to_snake_case();

        // Add rename attribute in case of aliasing
        if name.as_str() != self.1.as_ref() {
            context.attribute_field(format!(
                r#"serde(rename = "{original_name}")"#,
                original_name = self.1
            ));
        }

        Ok(RSFieldName(name.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "foo".into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @r#"RSFieldName("foo")"#
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "fooBar".into())
                .convert(&mut context)
                .unwrap(),
            @r#"RSFieldName("foo_bar")"#
        );
        assert_ron_snapshot!(
            context.drain_field_attributes(),
            @r#"
        [
          RSAttribute("serde(rename = \"fooBar\")"),
        ]
        "#
        )
    }

    #[test]
    fn test_convert_keyword() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "type".into())
                .convert(&mut context)
                .unwrap(),
            @r#"RSFieldName("type")"#
        );
        assert_ron_snapshot!(context.drain_field_attributes(), @"[]")
    }
}
