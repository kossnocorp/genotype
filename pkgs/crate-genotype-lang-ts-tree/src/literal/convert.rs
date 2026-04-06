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
    use crate::test::*;
    use genotype_test::Gt;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::literal_null()),
            @"Null"
        );
        assert_ron_snapshot!(
            convert_node(Gt::literal_boolean(true)),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::literal_integer(-123)),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::literal_float(1.23)),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::literal_string("Hello, world!")),
            @r#"String("Hello, world!")"#
        );
    }
}
