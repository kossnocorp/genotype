use genotype_lang_py_tree::{PYDefinition, PYReference};

use super::PYConvertContext;

impl PYConvertContext {
    pub fn hoist<HoistFn>(&mut self, mut hoist_fn: HoistFn) -> PYReference
    where
        HoistFn: FnMut(&mut PYConvertContext) -> PYDefinition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self);
        let reference = PYReference::new(definition.name().clone(), true);
        self.hoisted.push(definition);
        self.hoisting = false;
        self.track_reference(&reference);
        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<PYDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}
