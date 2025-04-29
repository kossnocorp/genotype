use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinition {
    Alias(TSAlias),
    Interface(TSInterface),
    Branded(TSBranded),
}

impl TSDefinition {
    pub fn name(&self) -> TSIdentifier {
        match self {
            TSDefinition::Alias(alias) => alias.name.clone(),
            TSDefinition::Interface(interface) => interface.name.clone(),
            TSDefinition::Branded(branded) => branded.name.clone(),
        }
    }
}

impl Into<TSDefinition> for TSBranded {
    fn into(self) -> TSDefinition {
        TSDefinition::Branded(self)
    }
}
