use crate::prelude::internal::*;

impl RsConvert<RsReference> for GtReference {
    fn convert(&self, context: &mut RsConvertContext) -> RsConvertResult<RsReference> {
        let identifier = self.identifier.convert(context)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.descriptor.convert(context))
            .collect::<RsConvertResult<Vec<_>>>()?;
        let definition_id = if context.is_generic_parameter(&identifier) {
            context.generic_definition_id(&identifier)
        } else {
            let Some(definition_id) = context.resolve_reference_definition_id(self) else {
                return Err(RsConvertError::UnresolvedReference(self.span));
            };
            definition_id
        };

        Ok(RsReference {
            id: self.id.clone(),
            identifier,
            arguments,
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
          arguments: [],
          definition_id: GtDefinitionId(GtModuleId("module"), "Name"),
        )
        "#,
        );
    }

    #[test]
    fn test_convert_with_arguments() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![(Gt::reference_id((0, 0)), Gt::definition_id("Response"))],
        );
        let reference = GtReference {
            arguments: vec![Gt::primitive_string().into()],
            ..Gt::reference_anon("Response")
        };

        assert_ron_snapshot!(
            convert_node_with(reference, &mut context),
            @r#"
        RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
          identifier: RsIdentifier("Response"),
          arguments: [
            Primitive(String),
          ],
          definition_id: GtDefinitionId(GtModuleId("module"), "Response"),
        )
        "#,
        );
    }

    #[test]
    fn test_convert_generic_parameter() {
        let mut context = Rst::convert_context();
        context.enter_generics_scope(vec!["Payload".into()]);

        assert_ron_snapshot!(
            convert_node_with(Gt::reference_anon("Payload"), &mut context),
            @r#"
        RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
          identifier: RsIdentifier("Payload"),
          arguments: [],
          definition_id: GtDefinitionId(GtModuleId("module"), "Payload"),
        )
        "#,
        );
    }
}
