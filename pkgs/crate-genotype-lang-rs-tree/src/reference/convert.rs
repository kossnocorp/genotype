use crate::prelude::internal::*;

impl RsConvert<RsReference> for GtReference {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsReference> {
        let identifier = self.identifier.convert(context)?;
        let Some(definition_id) = context.resolve_reference_definition_id(self) else {
            return Err(RsConverterError::UnresolvedReference(self.span.clone()).into());
        };

        Ok(RsReference {
            id: self.id.clone(),
            identifier,
            definition_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![(Gt::reference_id((0, 0)), Gt::definition_id("Name"))],
        );
        context.push_defined(&"Name".into());
        assert_ron_snapshot!(
            convert_node_with(Gt::reference_anon("Name"), &mut context),
            @r#"
        RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
          identifier: RsIdentifier("Name"),
          definition_id: GtDefinitionId(GtModuleId("module"), "Name"),
        )
        "#,
        );
    }
}
