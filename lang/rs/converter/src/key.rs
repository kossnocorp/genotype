use genotype_lang_rs_tree::field_name::RSFieldName;
use genotype_parser::tree::key::GTKey;
use heck::ToSnakeCase;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSFieldName> for GTKey {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSFieldName> {
        let name = self.1.to_snake_case();

        // Add rename attribute in case of aliasing
        if name != self.1 {
            context.attribute_field(format!(
                r#"serde(rename = "{original_name}")"#,
                original_name = self.1
            ));
        }

        Ok(RSFieldName(name))
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::RSAttribute;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSFieldName("foo".into()),
            GTKey::new((0, 0).into(), "foo".into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            RSFieldName("foo_bar".into()),
            GTKey::new((0, 0).into(), "fooBar".into())
                .convert(&mut context)
                .unwrap(),
        );
        assert_eq!(
            context.drain_field_attributes(),
            vec![RSAttribute(r#"serde(rename = "fooBar")"#.into())]
        )
    }

    #[test]
    fn test_convert_keyword() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTKey::new((0, 0).into(), "type".into())
                .convert(&mut context)
                .unwrap(),
            RSFieldName("type".into()),
        );
        assert_eq!(context.drain_field_attributes(), vec![])
    }
}
