use crate::prelude::internal::*;
use heck::ToSnakeCase;

impl PyConvert<PyKey> for GtKey {
    fn convert(&self, _context: &mut PyConvertContext) -> PyKey {
        PyKey(self.1.to_snake_case().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtKey::new((0, 0).into(), "foo".into()).convert(&mut PyConvertContext::default()),
            @r#"PyKey("foo")"#
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            GtKey::new((0, 0).into(), "fooBar".into()).convert(&mut context),
            @r#"PyKey("foo_bar")"#
        );
    }
}
