use genotype_lang_py_tree::key::PYKey;
use genotype_parser::tree::key::GTKey;
use heck::ToSnakeCase;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYKey> for GTKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYKey {
        PYKey(self.1.clone().to_snake_case())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYKey("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            PYKey("foo_bar".into()),
            GTKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
        );
    }
}
