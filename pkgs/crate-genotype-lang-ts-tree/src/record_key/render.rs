use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsRecordKey {
    fn render(
        &self,
        _state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        match context.mode() {
            TsMode::Effect => Ok(match self {
                TsRecordKey::Reference(reference) => {
                    reference.render(Default::default(), context)?
                }
                TsRecordKey::BooleanReference(_, _) | TsRecordKey::Boolean => {
                    "Schema.Literals([\"true\", \"false\"])".into()
                }
                TsRecordKey::String => "Schema.String".into(),
                TsRecordKey::Number => "Schema.Number".into(),
            }),
            TsMode::Zod => Ok(match self {
                TsRecordKey::Reference(reference) => {
                    reference.render(Default::default(), context)?
                }
                TsRecordKey::BooleanReference(_, _) => "z.enum([\"true\", \"false\"])".into(),
                TsRecordKey::String => "z.string()".into(),
                TsRecordKey::Number => "z.number()".into(),
                TsRecordKey::Boolean => "z.enum([\"true\", \"false\"])".into(),
            }),
            TsMode::Types => Ok(match self {
                TsRecordKey::Reference(reference) => {
                    reference.render(Default::default(), context)?
                }
                TsRecordKey::BooleanReference(reference, branded) => {
                    if *branded {
                        "`${boolean}`".into()
                    } else {
                        format!("`${{{}}}`", reference.render(Default::default(), context)?)
                    }
                }
                TsRecordKey::String => "string".into(),
                TsRecordKey::Number => "number".into(),
                TsRecordKey::Boolean => "`${boolean}`".into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::record_key_reference(Tst::reference("AddressId"))),
            @"AddressId"
        );

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
            @"`${boolean}`"
        );
    }

    #[test]
    fn test_render_zod() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::record_key_reference(Tst::reference("AddressId")), &mut context),
            @"AddressId"
        );

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
            @"z.enum([\"true\", \"false\"])"
        );
    }

    #[test]
    fn test_render_effect() {
        let mut context = Tst::render_context_effect();

        assert_snapshot!(
            render_node_with(Tst::record_key_reference(Tst::reference("AddressId")), &mut context),
            @"AddressId"
        );

        assert_snapshot!(
            render_node_with(Tst::record_key_string(), &mut context),
            @"Schema.String"
        );

        assert_snapshot!(
            render_node_with(Tst::record_key_number(), &mut context),
            @"Schema.Number"
        );

        assert_snapshot!(
            render_node_with(Tst::record_key_boolean(), &mut context),
            @"Schema.Literals([\"true\", \"false\"])"
        );
    }

    #[test]
    fn test_render_effect_reference() {
        assert_snapshot!(
            render_node_with(TsRecordKey::Reference(Tst::reference("Id")), &mut Tst::render_context_effect()),
            @r#"Id"#
        );
    }

    #[test]
    fn test_render_effect_boolean_reference_true() {
        assert_snapshot!(
            render_node_with(TsRecordKey::BooleanReference(Tst::reference("Flag"), true), &mut Tst::render_context_effect()),
            @r#"Schema.Literals(["true", "false"])"#
        );
    }

    #[test]
    fn test_render_effect_boolean_reference_false() {
        assert_snapshot!(
            render_node_with(TsRecordKey::BooleanReference(Tst::reference("Flag"), false), &mut Tst::render_context_effect()),
            @r#"Schema.Literals(["true", "false"])"#
        );
    }
}
