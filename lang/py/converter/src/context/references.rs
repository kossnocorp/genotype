use genotype_lang_py_tree::*;

use super::PYConvertContext;

impl PYConvertContext {
    /// Adds a reference to the context. This is used to track definition references.
    pub fn track_reference(&mut self, reference: &PYReference) {
        let mut references = self.references.pop().unwrap_or_default();
        references.insert(reference.identifier.clone());
        self.references.push(references);
    }

    /// Drains all references from the context. This is used to collect all references after
    /// converting a definition.
    pub fn pop_references_scope(&mut self) -> Vec<PYIdentifier> {
        self.references
            .pop()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    /// Creates a new reference scope. This is used to track references in nested aliases.
    pub fn create_references_scope(&mut self) {
        self.references.push(Default::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_reference() {
        let mut context = PYConvertContext::default();
        let reference = PYReference {
            identifier: "Foo".into(),
            forward: false,
        };
        context.track_reference(&reference);
        assert_eq!(context.pop_references_scope(), vec!["Foo".into()]);
    }

    #[test]
    fn test_track_reference_unique() {
        let mut context = PYConvertContext::default();
        context.track_reference(&PYReference {
            identifier: "Foo".into(),
            forward: false,
        });
        context.track_reference(&PYReference {
            identifier: "Foo".into(),
            forward: false,
        });
        assert_eq!(context.pop_references_scope(), vec!["Foo".into()]);
    }

    #[test]
    fn test_pop_references_scope() {
        let mut context = PYConvertContext::default();
        context.create_references_scope();
        context.track_reference(&PYReference {
            identifier: "Foo".into(),
            forward: false,
        });
        context.track_reference(&PYReference {
            identifier: "Bar".into(),
            forward: false,
        });
        assert_eq!(
            context.pop_references_scope(),
            vec!["Foo".into(), "Bar".into()]
        );
    }

    #[test]
    fn test_create_references_scope() {
        let mut context = PYConvertContext::default();
        context.create_references_scope();
        context.track_reference(&PYReference {
            identifier: "Foo".into(),
            forward: false,
        });
        context.create_references_scope();
        context.track_reference(&PYReference {
            identifier: "Bar".into(),
            forward: false,
        });
        assert_eq!(context.pop_references_scope(), vec!["Bar".into()]);
        assert_eq!(context.pop_references_scope(), vec!["Foo".into()]);
    }
}
