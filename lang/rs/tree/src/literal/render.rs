use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSLiteral;

impl GTRender for RSLiteral {
    fn render(&self, _indent: &GTIndent) -> String {
        let str = match self {
            RSLiteral::Boolean(value) => if *value { "True" } else { "False" }.to_string(),
            RSLiteral::Integer(value) => value.to_string(),
            RSLiteral::Float(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value)
                } else {
                    value.to_string()
                }
            }
            RSLiteral::String(value) => format!("\"{}\"", value.escape_default()),
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
            RSLiteral::Boolean(true).render(&rs_indent()),
            "Literal[True]"
        );
        assert_eq!(
            RSLiteral::Boolean(false).render(&rs_indent()),
            "Literal[False]"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_eq!(RSLiteral::Integer(1).render(&rs_indent()), "Literal[1]");
        assert_eq!(RSLiteral::Integer(-1).render(&rs_indent()), "Literal[-1]");
    }

    #[test]
    fn test_render_float() {
        assert_eq!(RSLiteral::Float(1.0).render(&rs_indent()), "Literal[1.0]");
        assert_eq!(RSLiteral::Float(-1.1).render(&rs_indent()), "Literal[-1.1]");
        assert_eq!(
            RSLiteral::Float(1.23456789).render(&rs_indent()),
            "Literal[1.23456789]"
        );
    }

    #[test]
    fn test_render_string() {
        assert_eq!(
            RSLiteral::String("Hi!".into()).render(&rs_indent()),
            "Literal[\"Hi!\"]"
        );
        assert_eq!(
            RSLiteral::String("Hello, \"world\"!\\".into()).render(&rs_indent()),
            "Literal[\"Hello, \\\"world\\\"!\\\\\"]"
        );
    }
}
