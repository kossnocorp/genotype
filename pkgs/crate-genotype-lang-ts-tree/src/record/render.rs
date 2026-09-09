use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsRecord {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        Ok(match context.mode() {
            TsMode::Effect => format!("Schema.Record({key}, {descriptor})"),
            TsMode::Zod => format!("z.record({key}, {descriptor})"),
            TsMode::Types => format!("Record<{key}, {descriptor}>"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(
                Tst::record(Tst::record_key_number(), Tst::primitive_string()),
            ),
            @"Record<number, string>"
        );
        assert_snapshot!(
            render_node(Tst::record(
                Tst::record_key_boolean(),
                Tst::primitive_string(),
            )),
            @"Record<`${boolean}`, string>"
        );
        assert_snapshot!(
            render_node(Tst::record(
                TsRecordKey::Reference(Tst::reference("AddressId")),
                TsReference::new("Address".into(), vec![], TsReferenceRel::Regular),
            )),
            @"Record<AddressId, Address>"
        );
    }

    #[test]
    fn test_render_zod() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::record(Tst::record_key_number(), Tst::primitive_string()),
                &mut context,
            ),
            @"z.record(z.number(), z.string())"
        );
        assert_snapshot!(
            render_node_with(
                Tst::record(Tst::record_key_boolean(), Tst::primitive_string()),
                &mut context,
            ),
            @"z.record(z.enum([\"true\", \"false\"]), z.string())"
        );
    }

    #[test]
    fn test_render_effect() {
        let mut context = Tst::render_context_effect();

        assert_snapshot!(
            render_node_with(
                Tst::record(Tst::record_key_number(), Tst::primitive_string()),
                &mut context,
            ),
            @"Schema.Record(Schema.Number, Schema.String)"
        );
        assert_snapshot!(
            render_node_with(
                Tst::record(Tst::record_key_boolean(), Tst::primitive_string()),
                &mut context,
            ),
            @"Schema.Record(Schema.Literals([\"true\", \"false\"]), Schema.String)"
        );
    }

    #[test]
    fn test_render_effect_reference_key() {
        assert_snapshot!(
            render_node_with(Tst::record(TsRecordKey::Reference(Tst::reference("Id")), Tst::reference("Value")), &mut Tst::render_context_effect()),
            @r#"Schema.Record(Id, Value)"#
        );
    }
}
