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
            GTPrimitive::Null(_) => TSPrimitive::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTPrimitive::Boolean((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int8((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Int16((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Int32((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Int64((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Int128((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::BigInt
        );
        assert_eq!(
            GTPrimitive::IntSize((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::IntU8((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::IntU16((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::IntU32((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::IntU64((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::IntU128((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::BigInt
        );
        assert_eq!(
            GTPrimitive::IntUSize((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Float32((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Null
        );
    }
}
