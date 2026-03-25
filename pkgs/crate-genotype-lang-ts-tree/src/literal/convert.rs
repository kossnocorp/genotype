use crate::prelude::internal::*;

impl TsConvert<TsLiteral> for GtLiteral {
    fn convert(&self, _context: &mut TsConvertContext) -> TsLiteral {
        match &self.value {
            GtLiteralValue::Null => TsLiteral::Null,
            GtLiteralValue::Boolean(value) => TsLiteral::Boolean(*value),
            GtLiteralValue::Integer(value) => TsLiteral::Integer(*value),
            GtLiteralValue::Float(value) => TsLiteral::Float(*value),
            GtLiteralValue::String(value) => TsLiteral::String(value.clone()),
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
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Null,
            }
            .convert(&mut Default::default()),
            @"Null"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Boolean(true),
            }
            .convert(&mut Default::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Integer(-123),
            }
            .convert(&mut Default::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Float(1.23),
            }
            .convert(&mut Default::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::String("Hello, world!".into()),
            }
            .convert(&mut Default::default()),
            @r#"String("Hello, world!")"#
        );
    }
}
