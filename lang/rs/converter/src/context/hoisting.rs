use genotype_lang_rs_tree::{RSDefinition, RSReference};
use miette::Result;

use super::{naming::RSContextParent, RSConvertContext};

impl RSConvertContext {
    pub fn hoist<HoistFn, HoistFnResult>(&mut self, mut hoist_fn: HoistFn) -> Result<RSReference>
    where
        HoistFnResult: Into<RSDefinition>,
        HoistFn: FnMut(&mut RSConvertContext) -> Result<HoistFnResult>,
    {
        self.hoisting = true;
        self.enter_parent(RSContextParent::Hoist);
        let definition = hoist_fn(self)?.into();
        let reference = RSReference::new(definition.name().clone(), definition.id().clone());
        self.hoisted.push(definition);
        self.exit_parent();
        self.hoisting = false;
        Ok(reference)
    }

    pub fn drain_hoisted(&mut self) -> Vec<RSDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}
