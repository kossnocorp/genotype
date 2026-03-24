use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSLiteral {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_null() {
        assert_snapshot!(
            TSLiteral::Null
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"null"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_snapshot!(
            TSLiteral::Boolean(true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"true"
        );
        assert_snapshot!(
            TSLiteral::Boolean(false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"false"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_snapshot!(
            TSLiteral::Integer(1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1"
        );
        assert_snapshot!(
            TSLiteral::Integer(-1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"-1"
        );
    }

    #[test]
    fn test_render_float() {
        assert_snapshot!(
            TSLiteral::Float(1.0)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1.0"
        );
        assert_snapshot!(
            TSLiteral::Float(-1.1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"-1.1"
        );
        assert_snapshot!(
            TSLiteral::Float(1.23456789)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1.23456789"
        );
    }

    #[test]
    fn test_render_string() {
        assert_snapshot!(
            TSLiteral::String("Hi!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#""Hi!""#
        );
        assert_snapshot!(
            TSLiteral::String("Hello, \"world\"!\\".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#""Hello, \"world\"!\\""#
        );
    }
}
