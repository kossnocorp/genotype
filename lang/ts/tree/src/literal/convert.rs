use crate::prelude::internal::*;

impl TSConvert<TSLiteral> for GTLiteral {
    fn convert(&self, _context: &mut TSConvertContext) -> TSLiteral {
        match &self.value {
            GTLiteralValue::Null => TSLiteral::Null,
            GTLiteralValue::Boolean(value) => TSLiteral::Boolean(*value),
            GTLiteralValue::Integer(value) => TSLiteral::Integer(*value),
            GTLiteralValue::Float(value) => TSLiteral::Float(*value),
            GTLiteralValue::String(value) => TSLiteral::String(value.clone()),
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
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Null,
            }
            .convert(&mut Default::default()),
            @"Null"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Boolean(true),
            }
            .convert(&mut Default::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Integer(-123),
            }
            .convert(&mut Default::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Float(1.23),
            }
            .convert(&mut Default::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::String("Hello, world!".into()),
            }
            .convert(&mut Default::default()),
            @r#"String("Hello, world!")"#
        );
    }
}
