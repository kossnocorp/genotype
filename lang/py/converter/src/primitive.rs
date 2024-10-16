use genotype_lang_py_tree::{definition::PYDefinition, primitive::PYPrimitive};
use genotype_parser::tree::primitive::GTPrimitive;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYPrimitive> for GTPrimitive {
    fn convert<HoistFn>(&self, _resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYPrimitive
    where
        HoistFn: Fn(PYDefinition),
    {
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

    use crate::resolve::PYConvertResolve;

    use super::*;
    use genotype_parser::tree::primitive::GTPrimitive;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTPrimitive::Boolean((0, 0).into()).convert(&PYConvertResolve::new(), &|_| {}),
            PYPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into()).convert(&PYConvertResolve::new(), &|_| {}),
            PYPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int((0, 0).into()).convert(&PYConvertResolve::new(), &|_| {}),
            PYPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Float((0, 0).into()).convert(&PYConvertResolve::new(), &|_| {}),
            PYPrimitive::Float
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into()).convert(&PYConvertResolve::new(), &|_| {}),
            PYPrimitive::None
        );
    }
}
