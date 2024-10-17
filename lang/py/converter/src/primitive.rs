use genotype_lang_py_tree::primitive::PYPrimitive;
use genotype_parser::tree::primitive::GTPrimitive;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut PYConvertContext) -> PYPrimitive {
        match self {
            GTPrimitive::Boolean(_) => PYPrimitive::Boolean,
            GTPrimitive::String(_) => PYPrimitive::String,
            GTPrimitive::Int(_) => PYPrimitive::Int,
            GTPrimitive::Float(_) => PYPrimitive::Float,
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
            GTPrimitive::Int((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Float((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::Float
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYPrimitive::None
        );
    }
}
