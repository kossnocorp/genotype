use genotype_lang_py_tree::primitive::PYPrimitive;
use genotype_parser::tree::primitive::GTPrimitive;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut PYConvertContext) -> PYPrimitive {
        match self {
            GTPrimitive::Boolean(_) => PYPrimitive::Boolean,
            GTPrimitive::String(_) => PYPrimitive::String,
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
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::primitive::GTPrimitive;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTPrimitive::Boolean((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int8((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Int16((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Int32((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Int64((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Int128((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntSize((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntU8((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntU16((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntU32((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntU64((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntU128((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::IntUSize((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Float
        );
        assert_eq!(
            GTPrimitive::Float32((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Float
        );
        assert_eq!(
            GTPrimitive::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Float
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::None
        );
    }
}
