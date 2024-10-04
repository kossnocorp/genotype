use genotype_lang_ts_tree::definition::TSDefinition;

use crate::resolve::TSConvertResolve;

pub trait TSConvert<TSNode> {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSNode
    where
        HoistFn: Fn(TSDefinition);
}
