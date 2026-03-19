use crate::prelude::internal::*;

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert(&self, _context: &mut TSConvertContext) -> TSPrimitive {
        match self {
            GTPrimitive::Boolean(_) => TSPrimitive::Boolean,
            GTPrimitive::String(_) => TSPrimitive::String,
            GTPrimitive::Number(_) => TSPrimitive::Number,
            GTPrimitive::Int8(_) => TSPrimitive::Number,
            GTPrimitive::Int16(_) => TSPrimitive::Number,
            GTPrimitive::Int32(_) => TSPrimitive::Number,
            GTPrimitive::Int64(_) => TSPrimitive::Number,
            GTPrimitive::Int128(_) => TSPrimitive::BigInt,
            GTPrimitive::IntSize(_) => TSPrimitive::Number,
            GTPrimitive::IntU8(_) => TSPrimitive::Number,
            GTPrimitive::IntU16(_) => TSPrimitive::Number,
            GTPrimitive::IntU32(_) => TSPrimitive::Number,
            GTPrimitive::IntU64(_) => TSPrimitive::Number,
            GTPrimitive::IntU128(_) => TSPrimitive::BigInt,
            GTPrimitive::IntUSize(_) => TSPrimitive::Number,
            GTPrimitive::Float32(_) => TSPrimitive::Number,
            GTPrimitive::Float64(_) => TSPrimitive::Number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTPrimitive::Boolean((0, 0).into()).convert(&mut Default::default()),
            @"Boolean"
        );
        assert_ron_snapshot!(
            GTPrimitive::String((0, 0).into()).convert(&mut Default::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int8((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int16((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int32((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int64((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Int128((0, 0).into()).convert(&mut Default::default()),
            @"BigInt"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntSize((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU8((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU16((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU32((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU64((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntU128((0, 0).into()).convert(&mut Default::default()),
            @"BigInt"
        );
        assert_ron_snapshot!(
            GTPrimitive::IntUSize((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float32((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
    }
}
