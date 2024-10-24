use genotype_lang_ts_tree::{definition::TSDefinition, TSRecord};
use genotype_parser::{tree::tuple::GTTuple, GTRecord};

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSRecord> for GTRecord {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSRecord
    where
        HoistFn: Fn(TSDefinition),
    {
        TSRecord {
            key: self.key.convert(resolve, hoist),
            descriptor: self.descriptor.convert(resolve, hoist),
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
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
            }
        );
    }
}
