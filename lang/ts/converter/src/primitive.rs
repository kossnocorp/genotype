use genotype_lang_ts_tree::primitive::TSPrimitive;
use genotype_parser::tree::primitive::GTPrimitive;

use crate::convert::TSConvert;

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert(&self) -> TSPrimitive {
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
        assert_eq!(GTPrimitive::Boolean.convert(), TSPrimitive::Boolean);
        assert_eq!(GTPrimitive::String.convert(), TSPrimitive::String);
        assert_eq!(GTPrimitive::Int.convert(), TSPrimitive::Number);
        assert_eq!(GTPrimitive::Float.convert(), TSPrimitive::Number);
    }
}
