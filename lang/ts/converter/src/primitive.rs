use genotype_lang_ts_tree::primitive::TSPrimitive;
use genotype_parser::tree::primitive::GTPrimitive;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert(&self, _context: &mut TSConvertContext) -> TSPrimitive {
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

    use super::*;
    use genotype_parser::tree::primitive::GTPrimitive;

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
            GTPrimitive::Int((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Float((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Number
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into()).convert(&mut Default::default()),
            TSPrimitive::Null
        );
    }
}
