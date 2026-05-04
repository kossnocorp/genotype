use crate::prelude::internal::*;

impl GtContext {
    pub fn enter_generics_scope(&mut self, generics: &Vec<GtGenericParameter>) {
        self.generics_scope.push(generics.clone());
    }

    pub fn exit_generics_scope(&mut self, span: GtSpan, node: GtNode) -> GtNodeParseResult<()> {
        self.generics_scope.pop().ok_or_else(|| {
            GtParseError::Internal(span, node, "tried to exit from an empty generics scope")
        })?;
        Ok(())
    }

    pub fn is_generic_parameter(&self, identifier: &GtIdentifier) -> bool {
        self.generics_scope.iter().any(|generics| {
            generics
                .iter()
                .any(|param| param.identifier.has_same_name(identifier))
        })
    }

    pub fn resolve_reference_identifier_as_generic_parameter(&mut self, identifier: &GtIdentifier) {
        if self.is_generic_parameter(&identifier) {
            self.mark_identifier_as_generic_parameter(&identifier);
        }
    }

    pub fn mark_identifier_as_generic_parameter(&mut self, identifier: &GtIdentifier) {
        self.resolve.generic_parameters.insert(identifier.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics_stack() {
        let mut context = GtContext::new("module".into());

        context.enter_generics_scope(&vec![Gt::generic_parameter("Q")]);

        let q = Gt::identifier_with_span("Q", (0, 1));
        let w = Gt::identifier_with_span("W", (0, 2));
        let e = Gt::identifier_with_span("E", (0, 3));

        assert!(context.is_generic_parameter(&q));
        assert!(!context.is_generic_parameter(&w));
        assert!(!context.is_generic_parameter(&e));

        context.enter_generics_scope(&vec![Gt::generic_parameter("W")]);

        assert!(context.is_generic_parameter(&q));
        assert!(context.is_generic_parameter(&w));
        assert!(!context.is_generic_parameter(&e));

        context
            .exit_generics_scope((0, 0).into(), GtNode::Alias)
            .unwrap();

        assert!(context.is_generic_parameter(&q));
        assert!(!context.is_generic_parameter(&w));
        assert!(!context.is_generic_parameter(&e));

        context
            .exit_generics_scope((0, 0).into(), GtNode::Alias)
            .unwrap();

        assert!(!context.is_generic_parameter(&q));
        assert!(!context.is_generic_parameter(&w));
        assert!(!context.is_generic_parameter(&e));
    }

    #[test]
    fn test_generics_stack_pop_error() {
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            context.exit_generics_scope((0, 0).into(), GtNode::Alias).unwrap_err(),
            @r#"Internal(GtSpan(0, 0), Alias, "tried to exit from an empty generics scope")"#
        );
    }

    #[test]
    fn test_resolve_reference_identifier_as_generic_parameter() {
        let mut context = GtContext::new("module".into());
        context.enter_generics_scope(&vec![Gt::generic_parameter("Q")]);
        context.resolve_reference_identifier_as_generic_parameter(&Gt::identifier_with_span(
            "Q",
            (0, 1),
        ));
        context.resolve_reference_identifier_as_generic_parameter(&Gt::identifier("W"));
        assert_ron_snapshot!(
            context.resolve.generic_parameters,
            @r#"
        [
          GtIdentifier(GtSpan(0, 1), "Q"),
        ]
        "#
        );
    }

    #[test]
    fn test_mark_identifier_as_generic_parameter() {
        let mut context = GtContext::new("module".into());
        let identifier = Gt::identifier("Q");
        context.mark_identifier_as_generic_parameter(&identifier);
        assert!(context.resolve.generic_parameters.contains(&identifier));
    }
}
