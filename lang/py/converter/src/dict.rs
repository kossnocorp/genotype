use genotype_lang_py_tree::{PYContextResolve, PYDict};
use genotype_parser::GTRecord;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDict> for GTRecord {
    fn convert(&self, context: &mut PYConvertContext) -> PYDict {
        PYDict {
            key: self.key.convert(context),
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
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
            .convert(&mut PYConvertContext::default()),
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            }
        );
    }
}
