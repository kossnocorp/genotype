use genotype_lang_rs_tree::{RSDefinition, RSReference};
use genotype_parser::GTSpan;
use miette::Result;

use super::{naming::RSContextParent, RSConvertContext};

impl RSConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> Result<RSReference>
    where
        Definition: Into<RSDefinition>,
        HoistFn: FnMut(&mut RSConvertContext) -> Result<(Definition, GTSpan)>,
    {
        self.hoisting = true;
        self.enter_parent(RSContextParent::Hoist);
        let (definition, span) = hoist_fn(self)?.into();
        let definition = definition.into();
        let reference = RSReference {
            id: self.reference_id(span),
            identifier: definition.name().clone(),
            definition_id: definition.id().clone(),
        };
        self.hoisted.push(definition.into());
        self.exit_parent();
        self.hoisting = false;
        Ok(reference)
    }

    pub fn drain_hoisted(&mut self) -> Vec<RSDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}
