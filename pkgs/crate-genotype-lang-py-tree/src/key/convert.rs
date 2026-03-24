use crate::prelude::internal::*;
use heck::ToSnakeCase;

impl PYConvert<PYKey> for GTKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYKey {
        PYKey(self.1.to_snake_case().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut PYConvertContext::default()),
            @r#"PYKey("foo")"#
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
            @r#"PYKey("foo_bar")"#
        );
    }
}
