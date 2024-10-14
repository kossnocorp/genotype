use genotype_lang_ts_tree::{definition::TSDefinition, union::TSUnion};
use genotype_parser::tree::union::GTUnion;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSUnion> for GTUnion {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSUnion
    where
        HoistFn: Fn(TSDefinition),
    {
        TSUnion {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(resolve, hoist))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTUnion {
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
