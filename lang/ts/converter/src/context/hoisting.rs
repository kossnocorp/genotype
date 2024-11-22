use genotype_lang_ts_tree::{TSDefinition, TSReference};

use super::TSConvertContext;

impl TSConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> TSReference
    where
        Definition: Into<TSDefinition>,
        HoistFn: FnMut(&mut TSConvertContext) -> Definition,
    {
        let definition = hoist_fn(self).into();
        let reference = TSReference(definition.name());
        self.hoisted.push(definition);
        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<TSDefinition> {
        self.hoisted.drain(..).collect()
    }
}
