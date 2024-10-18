use genotype_lang_py_tree::key::PYKey;
use genotype_parser::tree::key::GTKey;
use heck::ToSnakeCase;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYKey> for GTKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYKey {
        let name = self.1.clone().to_snake_case();
        if name == self.1 {
            return PYKey::new(name, None);
        } else {
            return PYKey::new(name, Some(self.1.clone()));
        }
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
            PYKey::new("foo".into(), None),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            PYKey::new("foo_bar".into(), Some("fooBar".into())),
            GTKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
        );
    }
}
