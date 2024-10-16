use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYLiteral;

impl GTRender for PYLiteral {
    fn render(&self, _indent: &GTIndent) -> String {
        let str = match self {
            PYLiteral::Boolean(value) => if *value { "True" } else { "False" }.to_string(),
            PYLiteral::Integer(value) => value.to_string(),
            PYLiteral::Float(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value)
                } else {
                    value.to_string()
                }
            }
            PYLiteral::String(value) => format!("\"{}\"", value.escape_default()),
        };
        format!("Literal[{str}]")
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_boolean() {
        assert_eq!(
            PYLiteral::Boolean(true).render(&py_indent()),
            "Literal[True]"
        );
        assert_eq!(
            PYLiteral::Boolean(false).render(&py_indent()),
            "Literal[False]"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_eq!(PYLiteral::Integer(1).render(&py_indent()), "Literal[1]");
        assert_eq!(PYLiteral::Integer(-1).render(&py_indent()), "Literal[-1]");
    }

    #[test]
    fn test_render_float() {
        assert_eq!(PYLiteral::Float(1.0).render(&py_indent()), "Literal[1.0]");
        assert_eq!(PYLiteral::Float(-1.1).render(&py_indent()), "Literal[-1.1]");
        assert_eq!(
            PYLiteral::Float(1.23456789).render(&py_indent()),
            "Literal[1.23456789]"
        );
    }

    #[test]
    fn test_render_string() {
        assert_eq!(
            PYLiteral::String("Hi!".into()).render(&py_indent()),
            "Literal[\"Hi!\"]"
        );
        assert_eq!(
            PYLiteral::String("Hello, \"world\"!\\".into()).render(&py_indent()),
            "Literal[\"Hello, \\\"world\\\"!\\\\\"]"
        );
    }
}
