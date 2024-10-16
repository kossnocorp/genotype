use genotype_lang_ts_tree::{definition::TSDefinition, primitive::TSPrimitive};
use genotype_parser::tree::primitive::GTPrimitive;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSPrimitive
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTPrimitive::Boolean(_) => TSPrimitive::Boolean,
            GTPrimitive::String(_) => TSPrimitive::String,
            GTPrimitive::Int(_) => TSPrimitive::Number,
            GTPrimitive::Float(_) => TSPrimitive::Number,
            GTPrimitive::Null(_) => TSPrimitive::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;
    use genotype_parser::tree::primitive::GTPrimitive;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTPrimitive::Boolean((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Float((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSPrimitive::Number
        );
    }
}
