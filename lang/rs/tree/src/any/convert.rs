use crate::prelude::internal::*;

impl RSConvert<RSAny> for GTAny {
    fn convert(&self, resolve: &mut RSConvertContext) -> Result<RSAny> {
        resolve.add_import(RSDependencyIdent::Runtime, "Any".into());
        Ok(RSAny)
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
            convert_to_rs(GtFactory::any()),
            @"RSAny"
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            convert_to_rs_with_context(GtFactory::any(), &mut context),
            @"RSAny"
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Runtime, RSIdentifier("Any")),
        ]
        "#
        );
    }
}
