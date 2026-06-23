use crate::prelude::internal::*;

impl RsConvertContext {
    pub fn provide_definition_generics(&mut self, generics: Vec<RsIdentifier>) {
        self.definition_generics = generics;
    }

    pub fn consume_definition_generics(&mut self) -> Vec<RsIdentifier> {
        std::mem::take(&mut self.definition_generics)
    }

    pub fn enter_generics_scope(&mut self, generics: Vec<RsIdentifier>) {
        self.generics_scope.push(generics);
    }

    pub fn exit_generics_scope(&mut self) {
        self.generics_scope.pop();
    }

    pub fn is_generic_parameter(&self, identifier: &RsIdentifier) -> bool {
        self.generics_scope
            .iter()
            .any(|generics| generics.iter().any(|generic| generic == identifier))
    }

    pub fn generic_definition_id(&self, identifier: &RsIdentifier) -> GtDefinitionId {
        GtDefinitionId(self.module_id.clone(), identifier.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_definition_generics() {
        let mut context = RsConvertContext::empty("module".into());
        context.provide_definition_generics(vec!["Payload".into(), "Error".into()]);

        assert_eq!(
            context.consume_definition_generics(),
            vec![RsIdentifier("Payload".into()), RsIdentifier("Error".into())]
        );
        assert_eq!(context.consume_definition_generics(), vec![]);
    }

    #[test]
    fn test_generics_scope() {
        let mut context = RsConvertContext::empty("module".into());

        assert!(!context.is_generic_parameter(&"Payload".into()));

        context.enter_generics_scope(vec!["Payload".into()]);

        assert!(context.is_generic_parameter(&"Payload".into()));
        assert!(!context.is_generic_parameter(&"Error".into()));
    }

    #[test]
    fn test_generics_scope_nested() {
        let mut context = RsConvertContext::empty("module".into());

        context.enter_generics_scope(vec!["Payload".into()]);
        context.enter_generics_scope(vec!["Error".into()]);

        assert!(context.is_generic_parameter(&"Payload".into()));
        assert!(context.is_generic_parameter(&"Error".into()));

        context.exit_generics_scope();

        assert!(context.is_generic_parameter(&"Payload".into()));
        assert!(!context.is_generic_parameter(&"Error".into()));
    }

    #[test]
    fn test_generic_definition_id() {
        let context = RsConvertContext::empty("module".into());

        assert_eq!(
            context.generic_definition_id(&"Payload".into()),
            GtDefinitionId("module".into(), "Payload".into())
        );
    }

    #[test]
    fn test_exit_empty_generics_scope() {
        let mut context = RsConvertContext::empty("module".into());

        context.exit_generics_scope();

        assert!(!context.is_generic_parameter(&"Payload".into()));
    }
}
