use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsLiteral {
    fn render(
        &self,
        _state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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

        match context.mode() {
            TsMode::Effect => Ok(if matches!(self, TsLiteral::Null) {
                "Schema.Null".into()
            } else {
                format!("Schema.Literal({literal})")
            }),
            TsMode::Zod => Ok(format!("z.literal({literal})")),
            TsMode::Types => Ok(literal),
        }
    }
}

#[cfg(test)]
mod tests {

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

    #[test]
    fn test_render_effect_null() {
        assert_snapshot!(
            render_node_with(Tst::literal_null(), &mut Tst::render_context_effect()),
            @r#"Schema.Null"#
        );
    }

    #[test]
    fn test_render_effect_boolean() {
        assert_snapshot!(
            render_node_with(Tst::literal_boolean(true), &mut Tst::render_context_effect()),
            @r#"Schema.Literal(true)"#
        );
    }

    #[test]
    fn test_render_effect_integer() {
        assert_snapshot!(
            render_node_with(Tst::literal_integer(-42), &mut Tst::render_context_effect()),
            @r#"Schema.Literal(-42)"#
        );
    }

    #[test]
    fn test_render_effect_float() {
        assert_snapshot!(
            render_node_with(Tst::literal_float(1.5), &mut Tst::render_context_effect()),
            @r#"Schema.Literal(1.5)"#
        );
    }

    #[test]
    fn test_render_effect_whole_float() {
        assert_snapshot!(
            render_node_with(Tst::literal_float(2.0), &mut Tst::render_context_effect()),
            @r#"Schema.Literal(2.0)"#
        );
    }

    #[test]
    fn test_render_effect_string() {
        assert_snapshot!(
            render_node_with(Tst::literal_string(r#"hello, "world""#), &mut Tst::render_context_effect()),
            @r#"Schema.Literal("hello, \"world\"")"#
        );
    }
}
