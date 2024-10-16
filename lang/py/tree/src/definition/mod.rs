use crate::{alias::PYAlias, interface::PYInterface};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDefinition {
    Alias(PYAlias),
    Interface(PYInterface),
}
