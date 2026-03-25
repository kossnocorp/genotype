use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyLiteral {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(&self, _state: PyRenderState, _context: &mut PyRenderContext) -> Result<String> {
        let str = match self {
            PyLiteral::None => "None".to_string(),

            PyLiteral::Boolean(value) => if *value { "True" } else { "False" }.to_string(),

            PyLiteral::Integer(value) => value.to_string(),

            PyLiteral::Float(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value)
                } else {
                    value.to_string()
                }
            }

            PyLiteral::String(value) => format!("\"{}\"", value.escape_default()),
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
            PyLiteral::None
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[None]"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_snapshot!(
            PyLiteral::Boolean(true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[True]"
        );
        assert_snapshot!(
            PyLiteral::Boolean(false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[False]"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_snapshot!(
            PyLiteral::Integer(1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1]"
        );
        assert_snapshot!(
            PyLiteral::Integer(-1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[-1]"
        );
    }

    #[test]
    fn test_render_float() {
        assert_snapshot!(
            PyLiteral::Float(1.0)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1.0]"
        );
        assert_snapshot!(
            PyLiteral::Float(-1.1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[-1.1]"
        );
        assert_snapshot!(
            PyLiteral::Float(1.23456789)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Literal[1.23456789]"
        );
    }

    #[test]
    fn test_render_string() {
        assert_snapshot!(
            PyLiteral::String("Hi!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#"Literal["Hi!"]"#
        );
        assert_snapshot!(
            PyLiteral::String("Hello, \"world\"!\\".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#"Literal["Hello, \"world\"!\\"]"#
        );
    }
}
