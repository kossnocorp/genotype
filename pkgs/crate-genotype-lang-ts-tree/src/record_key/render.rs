use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsRecordKey {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if context.is_zod_mode() {
            return Ok(match self {
                TsRecordKey::String => "z.string()".into(),
                TsRecordKey::Number => "z.number()".into(),
                TsRecordKey::Boolean => "z.boolean()".into(),
            });
        }

        Ok(match self {
            TsRecordKey::String => "string".into(),
            TsRecordKey::Number => "number".into(),
            TsRecordKey::Boolean => "boolean".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::record_key_string()),
            @"string"
        );
        assert_snapshot!(
            render_node(Tst::record_key_number()),
            @"number"
        );
        assert_snapshot!(
            render_node(Tst::record_key_boolean()),
            @"boolean"
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::record_key_string(), &mut context),
            @"z.string()"
        );

        assert_snapshot!(
            render_node_with(Tst::record_key_number(), &mut context),
            @"z.number()"
        );

        assert_snapshot!(
            render_node_with(Tst::record_key_boolean(), &mut context),
            @"z.boolean()"
        );
    }
}
