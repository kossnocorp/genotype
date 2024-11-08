use genotype_lang_rs_tree::key::RSKey;
use genotype_parser::tree::key::GTKey;
use heck::ToSnakeCase;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSKey> for GTKey {
    fn convert(&self, _context: &mut RSConvertContext) -> RSKey {
        RSKey(self.1.clone().to_snake_case())
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
            RSKey("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            RSKey("foo_bar".into()),
            GTKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
        );
    }
}
