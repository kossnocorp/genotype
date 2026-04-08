use crate::prelude::internal::*;

pub fn render_literal(literal: &GtLiteral) -> String {
    match &literal.value {
        GtLiteralValue::Null => "null".to_string(),
        GtLiteralValue::Boolean(value) => value.to_string(),
        GtLiteralValue::Integer(value) => value.to_string(),
        GtLiteralValue::Float(value) => GtLiteralValue::render_float(value),
        GtLiteralValue::String(value) => GtLiteralValue::render_string(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_literal() {
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Null,
            }),
            "null"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Boolean(true),
            }),
            "true"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Boolean(false),
            }),
            "false"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Integer(42),
            }),
            "42"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Float(3.14),
            }),
            "3.14"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Float(3.1415),
            }),
            "3.1415"
        );
        assert_eq!(
            render_literal(&GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::String("Hello\nWorld".into()),
            }),
            "\"Hello\\nWorld\""
        );
    }
}
