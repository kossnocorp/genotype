use crate::prelude::internal::*;

impl TsConvert<TsReference> for GtReference {
    fn convert(&self, context: &mut TsConvertContext) -> TsReference {
        TsReference::new(self.identifier.convert(context), TsReferenceRel::Regular)
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
            convert_node(Gt::reference("Name")),
            @r#"
        TsReference(
          identifier: TsIdentifier("Name"),
          rel: Regular,
        )
        "#,
        );
    }
}
