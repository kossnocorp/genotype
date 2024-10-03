use genotype_lang_ts_tree::{array::TSArray, definition::TSDefinition};
use genotype_parser::tree::array::GTArray;

use crate::convert::TSConvert;

impl TSConvert<TSArray> for GTArray {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSArray
    where
        HoistFn: Fn(TSDefinition),
    {
        TSArray {
            descriptor: self.descriptor.convert(hoist),
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
                descriptor: GTPrimitive::Boolean.into(),
            }
            .convert(&|_| {}),
            TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean)
            }
        );
    }
}
