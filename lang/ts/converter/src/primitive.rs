use genotype_lang_ts_tree::{definition::TSDefinition, primitive::TSPrimitive};
use genotype_parser::tree::primitive::GTPrimitive;

use crate::convert::TSConvert;

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert<HoistFn>(&self, _hoist: &HoistFn) -> TSPrimitive
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTPrimitive::Boolean => TSPrimitive::Boolean,
            GTPrimitive::String => TSPrimitive::String,
            GTPrimitive::Int => TSPrimitive::Number,
            GTPrimitive::Float => TSPrimitive::Number,
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
        assert_eq!(GTPrimitive::Boolean.convert(&|_| {}), TSPrimitive::Boolean);
        assert_eq!(GTPrimitive::String.convert(&|_| {}), TSPrimitive::String);
        assert_eq!(GTPrimitive::Int.convert(&|_| {}), TSPrimitive::Number);
        assert_eq!(GTPrimitive::Float.convert(&|_| {}), TSPrimitive::Number);
    }
}
