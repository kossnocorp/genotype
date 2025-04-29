use crate::prelude::internal::*;

impl TSConvert<TSArray> for GTArray {
    fn convert(&self, context: &mut TSConvertContext) -> TSArray {
        TSArray {
            descriptor: self.descriptor.convert(context),
        }
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
            .convert(&mut Default::default()),
            TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean)
            }
        );
    }
}
