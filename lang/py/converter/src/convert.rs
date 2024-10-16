use genotype_lang_py_tree::definition::PYDefinition;

use crate::resolve::PYConvertResolve;

pub trait PYConvert<PYNode> {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYNode
    where
        HoistFn: Fn(PYDefinition);
}
