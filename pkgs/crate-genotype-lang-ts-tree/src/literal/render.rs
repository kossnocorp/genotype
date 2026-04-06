use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsLiteral {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let literal = match self {
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
        };

        if context.is_zod_mode() {
            Ok(format!("z.literal({literal})"))
        } else {
            Ok(literal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_null() {
        assert_snapshot!(
            render_node(Tst::literal_null()),
            @"null"
        );
    }

    #[test]
    fn test_render_boolean() {
        assert_snapshot!(
            render_node(Tst::literal_boolean(true)),
            @"true"
        );
        assert_snapshot!(
            render_node(Tst::literal_boolean(false)),
            @"false"
        );
    }

    #[test]
    fn test_render_integer() {
        assert_snapshot!(
            render_node(Tst::literal_integer(1)),
            @"1"
        );
        assert_snapshot!(
            render_node(Tst::literal_integer(-1)),
            @"-1"
        );
    }

    #[test]
    fn test_render_float() {
        assert_snapshot!(
            render_node(Tst::literal_float(1.0)),
            @"1.0"
        );
        assert_snapshot!(
            render_node(Tst::literal_float(-1.1)),
            @"-1.1"
        );
        assert_snapshot!(
            render_node(Tst::literal_float(1.23456789)),
            @"1.23456789"
        );
    }

    #[test]
    fn test_render_string() {
        assert_snapshot!(
            render_node(Tst::literal_string("Hi!")),
            @r#""Hi!""#
        );
        assert_snapshot!(
            render_node(Tst::literal_string("Hello, \"world\"!\\")),
            @r#""Hello, \"world\"!\\""#
        );
    }

    #[test]
    fn test_render_literal_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::literal_null(), &mut context),
            @"z.literal(null)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_boolean(true), &mut context),
            @"z.literal(true)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_boolean(false), &mut context),
            @"z.literal(false)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_integer(1), &mut context),
            @"z.literal(1)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_integer(-1), &mut context),
            @"z.literal(-1)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_float(1.0), &mut context),
            @"z.literal(1.0)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_float(-1.1), &mut context),
            @"z.literal(-1.1)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_float(1.23456789), &mut context),
            @"z.literal(1.23456789)"
        );
        assert_snapshot!(
            render_node_with(Tst::literal_string("Hi!"), &mut context),
            @r#"z.literal("Hi!")"#
        );
        assert_snapshot!(
            render_node_with(Tst::literal_string("Hello, \"world\"!"), &mut context),
            @r#"z.literal("Hello, \"world\"!")"#
        );
    }
}
