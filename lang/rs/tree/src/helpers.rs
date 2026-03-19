use crate::prelude::internal::*;

pub fn render_literal(literal: &GTLiteral) -> String {
    match &literal.value {
        GTLiteralValue::Null => "null".to_string(),
        GTLiteralValue::Boolean(value) => value.to_string(),
        GTLiteralValue::Integer(value) => value.to_string(),
        GTLiteralValue::Float(value) => GTLiteralValue::render_float(&value),
        GTLiteralValue::String(value) => GTLiteralValue::render_string(&value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_literal() {
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Null,
            }),
            "null"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Boolean(true),
            }),
            "true"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Boolean(false),
            }),
            "false"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Integer(42),
            }),
            "42"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Float(3.14),
            }),
            "3.14"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Float(3.1415),
            }),
            "3.1415"
        );
        assert_eq!(
            render_literal(&GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::String("Hello\nWorld".into()),
            }),
            "\"Hello\\nWorld\""
        );
    }
}
