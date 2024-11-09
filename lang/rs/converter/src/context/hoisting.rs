use genotype_lang_rs_tree::{RSDefinition, RSReference};

use super::RSConvertContext;

impl RSConvertContext {
    pub fn hoist<HoistFn>(&mut self, mut hoist_fn: HoistFn) -> RSReference
    where
        HoistFn: FnMut(&mut RSConvertContext) -> RSDefinition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self);
        let reference = RSReference::new(definition.name().clone());
        self.hoisted.push(definition);
        self.hoisting = false;
        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<RSDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}
