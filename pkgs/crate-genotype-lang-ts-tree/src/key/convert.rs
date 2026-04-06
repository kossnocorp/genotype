use crate::prelude::internal::*;

impl TsConvert<TsKey> for GtKey {
    fn convert(&self, _context: &mut TsConvertContext) -> TsKey {
        TsKey(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(GtKey::new((0, 0).into(), "foo".into())),
            @r#"TsKey("foo")"#
        );
    }
}
