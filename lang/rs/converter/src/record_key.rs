use genotype_lang_rs_tree::{RSDescriptor, RSPrimitive};
use genotype_parser::GTRecordKey;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSDescriptor> for GTRecordKey {
    fn convert(&self, _context: &mut RSConvertContext) -> RSDescriptor {
        match self {
            GTRecordKey::String(_) => RSPrimitive::String.into(),
            GTRecordKey::Int(_) => RSPrimitive::Int.into(),
            GTRecordKey::Float(_) => RSPrimitive::Float32.into(),
            GTRecordKey::Boolean(_) => RSPrimitive::Boolean.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String),
            GTRecordKey::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into())),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int),
            GTRecordKey::Int((0, 0).into()).convert(&mut RSConvertContext::empty("module".into())),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Float32),
            GTRecordKey::Float((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into())),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean),
            GTRecordKey::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into())),
        );
    }
}
