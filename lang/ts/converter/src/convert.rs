use genotype_lang_ts_tree::definition::TSDefinition;

pub trait TSConvert<TSNode> {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSNode
    where
        HoistFn: Fn(TSDefinition);
}
