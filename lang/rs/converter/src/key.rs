use genotype_lang_rs_tree::field_name::RSFieldName;
use genotype_parser::tree::key::GTKey;
use heck::ToSnakeCase;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSFieldName> for GTKey {
    fn convert(&self, _context: &mut RSConvertContext) -> RSFieldName {
        RSFieldName(self.1.clone().to_snake_case())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSFieldName("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            RSFieldName("foo_bar".into()),
            GTKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
        );
    }
}
