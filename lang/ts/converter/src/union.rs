use genotype_lang_ts_tree::{definition::TSDefinition, union::TSUnion};
use genotype_parser::tree::union::GTUnion;

use crate::convert::TSConvert;

impl TSConvert<TSUnion> for GTUnion {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSUnion
    where
        HoistFn: Fn(TSDefinition),
    {
        TSUnion {
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
            GTUnion {
                descriptors: vec![
                    GTDescriptor::Primitive(GTPrimitive::Boolean),
                    GTDescriptor::Primitive(GTPrimitive::String),
                ]
            }
            .convert(&|_| {}),
            TSUnion {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
