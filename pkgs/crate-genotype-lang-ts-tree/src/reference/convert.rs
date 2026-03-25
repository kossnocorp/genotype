use crate::prelude::internal::*;

impl TsConvert<TsReference> for GtReference {
    fn convert(&self, context: &mut TsConvertContext) -> TsReference {
        TsReference(self.identifier.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            Gt::reference("Name").convert(&mut Default::default()),
            @r#"TsReference(TsIdentifier("Name"))"#,
        );
    }
}
