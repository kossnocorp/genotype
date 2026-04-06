use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsPrimitive {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if context.is_zod_mode() {
            return Ok(match self {
                TsPrimitive::String => "z.string()",
                TsPrimitive::Number => "z.number()",
                TsPrimitive::Boolean => "z.boolean()",
                TsPrimitive::BigInt => "z.bigint()",
                TsPrimitive::Null => "z.null()",
                TsPrimitive::Undefined => "z.undefined()",
            }
            .to_string());
        }

        Ok(match self {
            TsPrimitive::String => "string",
            TsPrimitive::Number => "number",
            TsPrimitive::Boolean => "boolean",
            TsPrimitive::BigInt => "bigint",
            TsPrimitive::Null => "null",
            TsPrimitive::Undefined => "undefined",
        }
        .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            render_node(Tst::primitive_string()),
            @"string"
        );
        assert_snapshot!(
            render_node(Tst::primitive_number()),
            @"number"
        );
        assert_snapshot!(
            render_node(Tst::primitive_bigint()),
            @"bigint"
        );
        assert_snapshot!(
            render_node(Tst::primitive_boolean()),
            @"boolean"
        );
        assert_snapshot!(
            render_node(Tst::primitive_null()),
            @"null"
        );
        assert_snapshot!(
            render_node(Tst::primitive_undefined()),
            @"undefined"
        );
    }

    #[test]
    fn test_render_primitive_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::primitive_string(), &mut context),
            @"z.string()"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_number(), &mut context),
            @"z.number()"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_bigint(), &mut context),
            @"z.bigint()"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_boolean(), &mut context),
            @"z.boolean()"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_null(), &mut context),
            @"z.null()"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_undefined(), &mut context),
            @"z.undefined()"
        );
    }
}
