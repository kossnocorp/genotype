use crate::prelude::internal::*;

impl RSConvert<RSPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut RSConvertContext) -> Result<RSPrimitive> {
        Ok(match self {
            GTPrimitive::Boolean(_) => RSPrimitive::Boolean,
            GTPrimitive::String(_) => RSPrimitive::String,
            GTPrimitive::Number(_) => RSPrimitive::Float64,
            GTPrimitive::Int8(_) => RSPrimitive::Int8,
            GTPrimitive::Int16(_) => RSPrimitive::Int16,
            GTPrimitive::Int32(_) => RSPrimitive::Int32,
            GTPrimitive::Int64(_) => RSPrimitive::Int64,
            GTPrimitive::Int128(_) => RSPrimitive::Int128,
            GTPrimitive::IntSize(_) => RSPrimitive::IntSize,
            GTPrimitive::IntU8(_) => RSPrimitive::IntU8,
            GTPrimitive::IntU16(_) => RSPrimitive::IntU16,
            GTPrimitive::IntU32(_) => RSPrimitive::IntU32,
            GTPrimitive::IntU64(_) => RSPrimitive::IntU64,
            GTPrimitive::IntU128(_) => RSPrimitive::IntU128,
            GTPrimitive::IntUSize(_) => RSPrimitive::IntUSize,
            GTPrimitive::Float32(_) => RSPrimitive::Float32,
            GTPrimitive::Float64(_) => RSPrimitive::Float64,
            GTPrimitive::Null(_) => RSPrimitive::Unit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTPrimitive::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int8
        );
        assert_eq!(
            GTPrimitive::Int16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int16
        );
        assert_eq!(
            GTPrimitive::Int32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int32
        );
        assert_eq!(
            GTPrimitive::Int64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int64
        );
        assert_eq!(
            GTPrimitive::Int128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int128
        );
        assert_eq!(
            GTPrimitive::IntSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntSize
        );
        assert_eq!(
            GTPrimitive::IntU8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntU8
        );
        assert_eq!(
            GTPrimitive::IntU16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntU16
        );
        assert_eq!(
            GTPrimitive::IntU32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntU32
        );
        assert_eq!(
            GTPrimitive::IntU64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntU64
        );
        assert_eq!(
            GTPrimitive::IntU128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntU128
        );
        assert_eq!(
            GTPrimitive::IntUSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::IntUSize
        );
        assert_eq!(
            GTPrimitive::Float32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Float32
        );
        assert_eq!(
            GTPrimitive::Float64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Float64
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Unit
        );
    }
}
