use crate::prelude::internal::*;

pub fn render_literal(literal: &GTLiteral) -> String {
    match literal {
        GTLiteral::Null(_) => "null".to_string(),
        GTLiteral::Boolean(_, value) => value.to_string(),
        GTLiteral::Integer(_, value) => value.to_string(),
        GTLiteral::Float(_, value) => GTLiteral::render_float(&value),
        GTLiteral::String(_, value) => GTLiteral::render_string(&value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_literal() {
        assert_eq!(render_literal(&GTLiteral::Null((0, 0).into())), "null");
        assert_eq!(
            render_literal(&GTLiteral::Boolean((0, 0).into(), true)),
            "true"
        );
        assert_eq!(
            render_literal(&GTLiteral::Boolean((0, 0).into(), false)),
            "false"
        );
        assert_eq!(render_literal(&GTLiteral::Integer((0, 0).into(), 42)), "42");
        assert_eq!(
            render_literal(&GTLiteral::Float((0, 0).into(), 3.14)),
            "3.14"
        );
        assert_eq!(
            render_literal(&GTLiteral::Float((0, 0).into(), 3.1415)),
            "3.1415"
        );
        assert_eq!(
            render_literal(&GTLiteral::String((0, 0).into(), "Hello\nWorld".into())),
            "\"Hello\\nWorld\""
        );
    }
}
