use crate::prelude::internal::*;

impl TSConvert<TSLiteral> for GTLiteral {
    fn convert(&self, _context: &mut TSConvertContext) -> TSLiteral {
        match self {
            GTLiteral::Null(_) => TSLiteral::Null,
            GTLiteral::Boolean(_, value) => TSLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => TSLiteral::Integer(*value),
            GTLiteral::Float(_, value) => TSLiteral::Float(*value),
            GTLiteral::String(_, value) => TSLiteral::String(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTLiteral::Null(Default::default()).convert(&mut Default::default()),
            @"Null"
        );
        assert_ron_snapshot!(
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut Default::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GTLiteral::Integer((0, 0).into(), -123).convert(&mut Default::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GTLiteral::Float((0, 0).into(), 1.23).convert(&mut Default::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&mut Default::default()),
            @r#"String("Hello, world!")"#
        );
    }
}
