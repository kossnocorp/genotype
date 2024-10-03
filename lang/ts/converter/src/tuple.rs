use genotype_lang_ts_tree::{definition::TSDefinition, tuple::TSTuple};
use genotype_parser::tree::tuple::GTTuple;

use crate::convert::TSConvert;

impl TSConvert<TSTuple> for GTTuple {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSTuple
    where
        HoistFn: Fn(TSDefinition),
    {
        TSTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(hoist))
                .collect(),
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
            GTTuple {
                descriptors: vec![GTPrimitive::Boolean.into(), GTPrimitive::String.into(),]
            }
            .convert(&|_| {}),
            TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
