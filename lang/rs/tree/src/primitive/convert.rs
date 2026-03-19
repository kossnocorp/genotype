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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTPrimitive::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Boolean"
        );
        assert_ron_snapshot!(
            GTPrimitive::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"String"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int8"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int16"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int32"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int64"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int128"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntSize"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU8((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU8"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU16((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU16"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU32"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU64"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU128((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU128"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntUSize((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntUSize"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float32((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Float32"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float64((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Float64"
        );
    }
}
