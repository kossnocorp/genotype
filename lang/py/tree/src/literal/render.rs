use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYLiteral {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _state: PYRenderState, _context: &mut PYRenderContext) -> Result<String> {
        let str = match self {
            PYLiteral::None => "None".to_string(),

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

        Ok(format!("Literal[{str}]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_none() {
        assert_eq!(
            PYLiteral::None
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[None]"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_eq!(
            PYLiteral::Boolean(true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[True]"
        );
        assert_eq!(
            PYLiteral::Boolean(false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[False]"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_eq!(
            PYLiteral::Integer(1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[1]"
        );
        assert_eq!(
            PYLiteral::Integer(-1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[-1]"
        );
    }

    #[test]
    fn test_render_float() {
        assert_eq!(
            PYLiteral::Float(1.0)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[1.0]"
        );
        assert_eq!(
            PYLiteral::Float(-1.1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[-1.1]"
        );
        assert_eq!(
            PYLiteral::Float(1.23456789)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[1.23456789]"
        );
    }

    #[test]
    fn test_render_string() {
        assert_eq!(
            PYLiteral::String("Hi!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[\"Hi!\"]"
        );
        assert_eq!(
            PYLiteral::String("Hello, \"world\"!\\".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Literal[\"Hello, \\\"world\\\"!\\\\\"]"
        );
    }
}
