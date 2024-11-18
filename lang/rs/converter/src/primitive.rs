use genotype_lang_rs_tree::primitive::RSPrimitive;
use genotype_parser::tree::primitive::GTPrimitive;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut RSConvertContext) -> Result<RSPrimitive> {
        Ok(match self {
            GTPrimitive::Boolean(_) => RSPrimitive::Boolean,
            GTPrimitive::String(_) => RSPrimitive::String,
            GTPrimitive::Int(_) => RSPrimitive::Int,
            GTPrimitive::Float(_) => RSPrimitive::Float32,
            GTPrimitive::Null(_) => RSPrimitive::Unit,
        })
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
            GTPrimitive::Boolean((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Boolean
        );
        assert_eq!(
            GTPrimitive::String((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::String
        );
        assert_eq!(
            GTPrimitive::Int((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Int
        );
        assert_eq!(
            GTPrimitive::Float((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Float32
        );
        assert_eq!(
            GTPrimitive::Null((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSPrimitive::Unit
        );
    }
}
