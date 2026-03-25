use crate::prelude::internal::*;

impl RsConvert<RsReference> for GtReference {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsReference> {
        let identifier = self.identifier.convert(context)?;
        let definition_id = match &self.definition_id {
            GtReferenceDefinitionId::Resolved(id) => id.clone(),
            GtReferenceDefinitionId::Unresolved => {
                return Err(RsConverterError::UnresolvedReference(self.span.clone()).into());
            }
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
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        let mut context = RsConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_ron_snapshot!(
            convert_node(Gt::reference("Name")),
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
