use genotype_lang_rs_tree::{RSContextResolve, RSDict};
use genotype_parser::GTRecord;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSDict> for GTRecord {
    fn convert(&self, context: &mut RSConvertContext) -> RSDict {
        RSDict {
            key: self.key.convert(context),
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
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
            .convert(&mut RSConvertContext::default()),
            RSDict {
                key: RSDictKey::String,
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
        );
    }
}
