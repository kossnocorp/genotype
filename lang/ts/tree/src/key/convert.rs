use crate::prelude::internal::*;

impl TSConvert<TSKey> for GTKey {
    fn convert(&self, _context: &mut TSConvertContext) -> TSKey {
        TSKey(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut Default::default()),
            @r#"TSKey("foo")"#
        );
    }
}
