use genotype_lang_rs_tree::{RSDescriptor, RSPrimitive};
use genotype_parser::GTRecordKey;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSDescriptor> for GTRecordKey {
    fn convert(&self, _context: &mut RSConvertContext) -> Result<RSDescriptor> {
        Ok(match self {
            GTRecordKey::String(_) => RSPrimitive::String.into(),
            GTRecordKey::Int8(_) => RSPrimitive::Int8.into(),
            GTRecordKey::Int16(_) => RSPrimitive::Int16.into(),
            GTRecordKey::Int32(_) => RSPrimitive::Int32.into(),
            GTRecordKey::Int64(_) => RSPrimitive::Int64.into(),
            GTRecordKey::Int128(_) => RSPrimitive::Int128.into(),
            GTRecordKey::IntSize(_) => RSPrimitive::IntSize.into(),
            GTRecordKey::IntU8(_) => RSPrimitive::IntU8.into(),
            GTRecordKey::IntU16(_) => RSPrimitive::IntU16.into(),
            GTRecordKey::IntU32(_) => RSPrimitive::IntU32.into(),
            GTRecordKey::IntU64(_) => RSPrimitive::IntU64.into(),
            GTRecordKey::IntU128(_) => RSPrimitive::IntU128.into(),
            GTRecordKey::IntUSize(_) => RSPrimitive::IntUSize.into(),
            GTRecordKey::Float32(_) => RSPrimitive::Float32.into(),
            GTRecordKey::Float64(_) => RSPrimitive::Float64.into(),
            GTRecordKey::Boolean(_) => RSPrimitive::Boolean.into(),
        })
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
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int8),
            GTRecordKey::Int8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int16),
            GTRecordKey::Int16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int32),
            GTRecordKey::Int32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int64),
            GTRecordKey::Int64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Int128),
            GTRecordKey::Int128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntSize),
            GTRecordKey::IntSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU8),
            GTRecordKey::IntU8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU16),
            GTRecordKey::IntU16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU32),
            GTRecordKey::IntU32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU64),
            GTRecordKey::IntU64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntU128),
            GTRecordKey::IntU128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::IntUSize),
            GTRecordKey::IntUSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Float32),
            GTRecordKey::Float32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Float64),
            GTRecordKey::Float64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean),
            GTRecordKey::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
    }
}
