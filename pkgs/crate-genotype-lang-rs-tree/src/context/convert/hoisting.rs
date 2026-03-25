use crate::prelude::internal::*;

impl RsConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> Result<RsReference>
    where
        Definition: Into<RsDefinition>,
        HoistFn: FnMut(&mut RsConvertContext) -> Result<(Definition, GtSpan)>,
    {
        self.hoisting = true;
        self.enter_parent(RsContextParent::Hoist);
        let (definition, span) = hoist_fn(self)?.into();
        let definition = definition.into();
        let reference = RsReference {
            id: self.reference_id(span),
            identifier: definition.name().clone(),
            definition_id: definition.id().clone(),
        };
        self.hoisted.push(definition.into());
        self.exit_parent();
        self.hoisting = false;
        Ok(reference)
    }

    pub fn drain_hoisted(&mut self) -> Vec<RsDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}
