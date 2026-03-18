use crate::prelude::internal::*;

impl PYConvert<PYPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut PYConvertContext) -> PYPrimitive {
        match self {
            GTPrimitive::Boolean(_) => PYPrimitive::Boolean,
            GTPrimitive::String(_) => PYPrimitive::String,
            GTPrimitive::Number(_) => PYPrimitive::Float,
            GTPrimitive::Int8(_) => PYPrimitive::Int,
            GTPrimitive::Int16(_) => PYPrimitive::Int,
            GTPrimitive::Int32(_) => PYPrimitive::Int,
            GTPrimitive::Int64(_) => PYPrimitive::Int,
            GTPrimitive::Int128(_) => PYPrimitive::Int,
            GTPrimitive::IntSize(_) => PYPrimitive::Int,
            GTPrimitive::IntU8(_) => PYPrimitive::Int,
            GTPrimitive::IntU16(_) => PYPrimitive::Int,
            GTPrimitive::IntU32(_) => PYPrimitive::Int,
            GTPrimitive::IntU64(_) => PYPrimitive::Int,
            GTPrimitive::IntU128(_) => PYPrimitive::Int,
            GTPrimitive::IntUSize(_) => PYPrimitive::Int,
            GTPrimitive::Float32(_) => PYPrimitive::Float,
            GTPrimitive::Float64(_) => PYPrimitive::Float,
            GTPrimitive::Null(_) => PYPrimitive::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTPrimitive::Boolean((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Boolean"
        );
        assert_ron_snapshot!(
            GTPrimitive::String((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GTPrimitive::Number((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int8((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int16((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int32((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int64((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int128((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntSize((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU8((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU16((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU32((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU64((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU128((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntUSize((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float32((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GTPrimitive::Null((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"r#None"
        );
    }
}
