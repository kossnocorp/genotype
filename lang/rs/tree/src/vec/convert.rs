use crate::prelude::internal::*;

impl RSConvert<RSVec> for GTArray {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSVec> {
        let descriptor = self.descriptor.convert(context)?;
        Ok(RSVec { descriptor })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtFactory::array(GtFactory::primitive_boolean())
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @"
        RSVec(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }
}
