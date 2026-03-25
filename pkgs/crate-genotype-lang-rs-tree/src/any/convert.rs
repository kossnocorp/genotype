use crate::prelude::internal::*;

impl RsConvert<RsAny> for GtAny {
    fn convert(&self, resolve: &mut RsConvertContext) -> Result<RsAny> {
        resolve.add_import(RsDependencyIdent::Runtime, "Any".into());
        Ok(RsAny)
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
            convert_node(Gt::any()),
            @"RsAny"
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            convert_node_with(Gt::any(), &mut context),
            @"RsAny"
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Runtime, RsIdentifier("Any")),
        ]
        "#
        );
    }
}
