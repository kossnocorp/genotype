use genotype_lang_ts_tree::array::TSArray;
use genotype_parser::tree::array::GTArray;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSArray> for GTArray {
    fn convert(&self, context: &mut TSConvertContext) -> TSArray {
        TSArray {
            descriptor: self.descriptor.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

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
