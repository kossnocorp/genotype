use crate::prelude::internal::*;

impl PYContextResolve for PYTuple {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if context.is_version(PYVersion::Legacy) {
            context.add_import(PYDependencyIdent::Typing, "Tuple".into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PYConvertContextMock::default();
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(context.as_imports(), @"[]");
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(
            context.as_imports(),
            @r#"
        [
          (Typing, PYIdentifier("Tuple")),
        ]
        "#
        );
    }
}
