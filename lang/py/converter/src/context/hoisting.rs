use genotype_lang_py_tree::*;

use super::PYConvertContext;

impl PYConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> PYReference
    where
        Definition: Into<PYDefinition>,
        HoistFn: FnMut(&mut PYConvertContext) -> Definition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self).into();
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
