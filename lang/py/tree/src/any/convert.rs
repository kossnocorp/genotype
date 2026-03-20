use crate::prelude::internal::*;

impl PYConvert<PYAny> for GTAny {
    fn convert(&self, resolve: &mut PYConvertContext) -> PYAny {
        PYAny.resolve(resolve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_py(GtFactory::any()),
            @"PYAny"
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            convert_to_py_with_context(GtFactory::any(), &mut context),
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
