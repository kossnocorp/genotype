use crate::prelude::internal::*;

impl TsConvert<TsDoc> for GtDoc {
    fn convert(&self, _context: &mut TsConvertContext) -> TsDoc {
        TsDoc(self.1.clone())
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
            convert_node(GtDoc((0, 0).into(), "Hello, world!".into())),
            @r#"TsDoc("Hello, world!")"#
        );
    }
}
