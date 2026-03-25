use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsLiteral {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TsLiteral::Null => "null".to_string(),
            TsLiteral::Boolean(value) => value.to_string(),
            TsLiteral::Integer(value) => value.to_string(),
            TsLiteral::Float(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value)
                } else {
                    value.to_string()
                }
            }
            TsLiteral::String(value) => format!("\"{}\"", value.escape_default()),
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
            TsLiteral::Null
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"null"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_snapshot!(
            TsLiteral::Boolean(true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"true"
        );
        assert_snapshot!(
            TsLiteral::Boolean(false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"false"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_snapshot!(
            TsLiteral::Integer(1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1"
        );
        assert_snapshot!(
            TsLiteral::Integer(-1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"-1"
        );
    }

    #[test]
    fn test_render_float() {
        assert_snapshot!(
            TsLiteral::Float(1.0)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1.0"
        );
        assert_snapshot!(
            TsLiteral::Float(-1.1)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"-1.1"
        );
        assert_snapshot!(
            TsLiteral::Float(1.23456789)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"1.23456789"
        );
    }

    #[test]
    fn test_render_string() {
        assert_snapshot!(
            TsLiteral::String("Hi!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#""Hi!""#
        );
        assert_snapshot!(
            TsLiteral::String("Hello, \"world\"!\\".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @r#""Hello, \"world\"!\\""#
        );
    }
}
