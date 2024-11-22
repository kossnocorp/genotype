use genotype_lang_ts_tree::TSRecord;
use genotype_parser::GTRecord;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSRecord> for GTRecord {
    fn convert(&self, context: &mut TSConvertContext) -> TSRecord {
        TSRecord {
            key: self.key.convert(context),
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
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
            }
        );
    }
}
