use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSLiteral;

impl GTRender for TSLiteral {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            TSLiteral::Null => "null".to_string(),
            TSLiteral::Boolean(value) => value.to_string(),
            TSLiteral::Integer(value) => value.to_string(),
            TSLiteral::Float(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value)
                } else {
                    value.to_string()
                }
            }
            TSLiteral::String(value) => format!("\"{}\"", value.escape_default()),
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_null() {
        assert_eq!(TSLiteral::Null.render(&ts_indent()), "null");
    }

    #[test]
    fn test_render_boolean() {
        assert_eq!(TSLiteral::Boolean(true).render(&ts_indent()), "true");
        assert_eq!(TSLiteral::Boolean(false).render(&ts_indent()), "false");
    }

    #[test]
    fn test_render_integer() {
        assert_eq!(TSLiteral::Integer(1).render(&ts_indent()), "1");
        assert_eq!(TSLiteral::Integer(-1).render(&ts_indent()), "-1");
    }

    #[test]
    fn test_render_float() {
        assert_eq!(TSLiteral::Float(1.0).render(&ts_indent()), "1.0");
        assert_eq!(TSLiteral::Float(-1.1).render(&ts_indent()), "-1.1");
        assert_eq!(
            TSLiteral::Float(1.23456789).render(&ts_indent()),
            "1.23456789"
        );
    }

    #[test]
    fn test_render_string() {
        assert_eq!(
            TSLiteral::String("Hi!".into()).render(&ts_indent()),
            "\"Hi!\""
        );
        assert_eq!(
            TSLiteral::String("Hello, \"world\"!\\".into()).render(&ts_indent()),
            "\"Hello, \\\"world\\\"!\\\\\""
        );
    }
}
