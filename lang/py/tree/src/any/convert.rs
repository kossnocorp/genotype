use crate::prelude::internal::*;

impl PYConvert<PYAny> for GTAny {
    fn convert(&self, resolve: &mut PYConvertContext) -> PYAny {
        PYAny.resolve(resolve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTAny((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"PYAny"
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTAny((0, 0).into(),).convert(&mut context),
            @"PYAny"
        );

        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Any")),
        ]
        "#
        );
    }
}
