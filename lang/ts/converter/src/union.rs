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
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTUnion {
                descriptors: vec![GTPrimitive::Boolean.into(), GTPrimitive::String.into(),]
            }
            .convert(&|_| {}),
            TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
