use crate::{alias::PYAlias, class::PYClass};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDefinition {
    Alias(PYAlias),
    Interface(PYClass),
}
