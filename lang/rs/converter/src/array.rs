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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean)
            }
        );
    }
}
