use crate::{alias::TSAlias, interface::TSInterface};

pub enum TSDefinitionDescriptor {
    Alias(TSAlias),
    Interface(TSInterface),
}
