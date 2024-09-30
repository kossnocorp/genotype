use crate::{alias::TSAlias, interface::TSInterface};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinitionDescriptor {
    Alias(TSAlias),
    Interface(TSInterface),
}
