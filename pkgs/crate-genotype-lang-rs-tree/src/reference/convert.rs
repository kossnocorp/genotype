use crate::prelude::internal::*;

impl RSConvert<RSReference> for GTReference {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSReference> {
        let identifier = self.identifier.convert(context)?;
        let definition_id = match &self.definition_id {
            GTReferenceDefinitionId::Resolved(id) => id.clone(),
            GTReferenceDefinitionId::Unresolved => {
                return Err(RSConverterError::UnresolvedReference(self.span.clone()).into());
            }
        };

        Ok(RSReference {
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
        let mut context = RSConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_ron_snapshot!(
            convert_node(Gt::reference("Name")),
            @r#"
        RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
          identifier: RSIdentifier("Name"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Name"),
        )
        "#,
        );
    }
}
