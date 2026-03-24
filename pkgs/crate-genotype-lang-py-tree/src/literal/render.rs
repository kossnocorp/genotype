use crate::prelude::internal::*;

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
    use insta::assert_snapshot;

    #[test]
    fn test_render_none() {
        assert_snapshot!(
            PYLiteral::None
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[None]"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_snapshot!(
            PYLiteral::Boolean(true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[True]"
        );
        assert_snapshot!(
            PYLiteral::Boolean(false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[False]"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_snapshot!(
            PYLiteral::Integer(1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1]"
        );
        assert_snapshot!(
            PYLiteral::Integer(-1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[-1]"
        );
    }

    #[test]
    fn test_render_float() {
        assert_snapshot!(
            PYLiteral::Float(1.0)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1.0]"
        );
        assert_snapshot!(
            PYLiteral::Float(-1.1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[-1.1]"
        );
        assert_snapshot!(
            PYLiteral::Float(1.23456789)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1.23456789]"
        );
    }

    #[test]
    fn test_render_string() {
        assert_snapshot!(
            PYLiteral::String("Hi!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#"Literal["Hi!"]"#
        );
        assert_snapshot!(
            PYLiteral::String("Hello, \"world\"!\\".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#"Literal["Hello, \"world\"!\\"]"#
        );
    }
}
