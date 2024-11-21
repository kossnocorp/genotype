use crate::{alias::TSAlias, interface::TSInterface, TSBranded};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinition {
    Alias(TSAlias),
    Interface(TSInterface),
    Branded(TSBranded),
}

impl Into<TSDefinition> for TSBranded {
    fn into(self) -> TSDefinition {
        TSDefinition::Branded(self)
    }
}
