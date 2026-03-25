use crate::prelude::internal::*;

impl RsConvert<RsVec> for GtArray {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsVec> {
        let descriptor = self.descriptor.convert(context)?;
        Ok(RsVec { descriptor })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            Gt::array(Gt::primitive_boolean())
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @"
        RsVec(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }
}
