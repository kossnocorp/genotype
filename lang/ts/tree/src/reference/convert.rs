use crate::prelude::internal::*;

impl TSConvert<TSReference> for GTReference {
    fn convert(&self, context: &mut TSConvertContext) -> TSReference {
        TSReference(self.identifier.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtFactory::reference("Name").convert(&mut Default::default()),
            @r#"TSReference(TSIdentifier("Name"))"#,
        );
    }
}
