use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsPrimitive {
    fn render(
        &self,
        _state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        match context.mode() {
            TsMode::Effect => Ok(match self {
                TsPrimitive::String => "Schema.String",
                TsPrimitive::Number => "Schema.Number",
                TsPrimitive::Boolean => "Schema.Boolean",
                TsPrimitive::BigInt => "Schema.BigInt",
                TsPrimitive::Null => "Schema.Null",
                TsPrimitive::Undefined => "Schema.Undefined",
            }
            .into()),
            TsMode::Zod => Ok(match self {
                TsPrimitive::String => "z.string()",
                TsPrimitive::Number => "z.number()",
                TsPrimitive::Boolean => "z.boolean()",
                TsPrimitive::BigInt => "z.bigint()",
                TsPrimitive::Null => "z.null()",
                TsPrimitive::Undefined => "z.undefined()",
            }
            .to_string()),
            TsMode::Types => Ok(match self {
                TsPrimitive::String => "string",
                TsPrimitive::Number => "number",
                TsPrimitive::Boolean => "boolean",
                TsPrimitive::BigInt => "bigint",
                TsPrimitive::Null => "null",
                TsPrimitive::Undefined => "undefined",
            }
            .to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_render_primitive_effect_mode() {
        let mut context = Tst::render_context_effect();

        assert_snapshot!(
            render_node_with(Tst::primitive_string(), &mut context),
            @"Schema.String"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_number(), &mut context),
            @"Schema.Number"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_bigint(), &mut context),
            @"Schema.BigInt"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_boolean(), &mut context),
            @"Schema.Boolean"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_null(), &mut context),
            @"Schema.Null"
        );
        assert_snapshot!(
            render_node_with(Tst::primitive_undefined(), &mut context),
            @"Schema.Undefined"
        );
    }
}
