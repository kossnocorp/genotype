use crate::prelude::internal::*;

impl PYConvert<PYDict> for GTRecord {
    fn convert(&self, context: &mut PYConvertContext) -> PYDict {
        PYDict {
            key: self.key.convert(context),
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            }
        );
    }
}
