use crate::{alias::TSAlias, interface::TSInterface};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinition {
    Alias(TSAlias),
    Interface(TSInterface),
}
