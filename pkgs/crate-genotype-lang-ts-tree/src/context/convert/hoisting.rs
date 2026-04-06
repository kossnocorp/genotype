use crate::prelude::internal::*;

impl TsConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> TsReference
    where
        Definition: Into<TsDefinition>,
        HoistFn: FnMut(&mut TsConvertContext) -> Definition,
    {
        let definition = hoist_fn(self).into();
        let reference = TsReference::new(definition.name(), TsReferenceRel::Regular);
        self.hoisted.push(definition);
        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<TsDefinition> {
        self.hoisted.drain(..).collect()
    }
}
