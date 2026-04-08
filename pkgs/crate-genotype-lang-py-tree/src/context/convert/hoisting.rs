use crate::prelude::internal::*;

impl PyConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> PyReference
    where
        Definition: Into<PyDefinition>,
        HoistFn: FnMut(&mut PyConvertContext) -> Definition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self).into();
        let reference = PyReference::new(definition.name().clone(), true);
        self.hoisted.push(definition);
        self.hoisting = false;
        self.track_reference(&reference);
        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<PyDefinition> {
        self.defined.append(&mut self.hoist_defined);
        self.hoisted.drain(..).collect()
    }
}
