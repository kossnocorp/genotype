use crate::prelude::internal::*;
use heck::ToSnakeCase;

impl PYConvert<PYKey> for GTKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYKey {
        PYKey(self.1.clone().to_snake_case())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
