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
    use genotype_lang_ts_tree::{primitive::TSPrimitive, type_descriptor::TSTypeDescriptor};
    use genotype_parser::tree::{descriptor::GTDescriptor, primitive::GTPrimitive};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTTuple {
                descriptors: vec![
                    GTDescriptor::Primitive(GTPrimitive::Boolean),
                    GTDescriptor::Primitive(GTPrimitive::String),
                ]
            }
            .convert(&|_| {}),
            TSTuple {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
