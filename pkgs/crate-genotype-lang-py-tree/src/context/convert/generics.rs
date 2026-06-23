use crate::prelude::internal::*;

impl PyConvertContext {
    pub fn provide_definition_generics(&mut self, generics: Vec<PyIdentifier>) {
        self.definition_generics = generics;
    }

    pub fn consume_definition_generics(&mut self) -> Vec<PyIdentifier> {
        std::mem::take(&mut self.definition_generics)
    }

    pub fn enter_generics_scope(&mut self, generics: Vec<PyIdentifier>) {
        self.generics_scope.push(generics);
    }

    pub fn exit_generics_scope(&mut self) {
        self.generics_scope.pop();
    }

    pub fn is_generic_parameter(&self, identifier: &PyIdentifier) -> bool {
        self.generics_scope
            .iter()
            .any(|generics| generics.iter().any(|generic| generic == identifier))
    }

    pub fn resolve_generics_imports(&mut self, generics: &[PyIdentifier], kind: PyGenericsKind) {
        if generics.is_empty() || !self.is_version(PyVersion::Legacy) {
            return;
        }

        context_push_type_var_import(self);
        match kind {
            PyGenericsKind::Alias => self.push_import(PyImport::new(
                PyDependencyIdent::TypingExtensions,
                "TypeAlias".into(),
            )),
            PyGenericsKind::Class => {
                self.push_import(PyImport::new(PyDependencyIdent::Typing, "Generic".into()))
            }
        }
    }
}

fn context_push_type_var_import(context: &mut PyConvertContext) {
    context.push_import(PyImport::new(PyDependencyIdent::Typing, "TypeVar".into()));
}

pub enum PyGenericsKind {
    Alias,
    Class,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_definition_generics() {
        let mut context = PyConvertContext::default();
        context.provide_definition_generics(vec!["Payload".into(), "Error".into()]);

        assert_eq!(
            context.consume_definition_generics(),
            vec![PyIdentifier("Payload".into()), PyIdentifier("Error".into())]
        );
        assert_eq!(context.consume_definition_generics(), vec![]);
    }

    #[test]
    fn test_generics_scope_nested() {
        let mut context = PyConvertContext::default();

        context.enter_generics_scope(vec!["Payload".into()]);
        context.enter_generics_scope(vec!["Error".into()]);

        assert!(context.is_generic_parameter(&"Payload".into()));
        assert!(context.is_generic_parameter(&"Error".into()));

        context.exit_generics_scope();

        assert!(context.is_generic_parameter(&"Payload".into()));
        assert!(!context.is_generic_parameter(&"Error".into()));
    }
}
