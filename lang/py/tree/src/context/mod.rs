use crate::{PYDependency, PYIdentifier, PYVersion};

#[cfg(test)]
pub mod mock;

pub trait PYContext {
    fn import(&mut self, dependency: PYDependency, name: PYIdentifier);

    fn is_version(&self, version: PYVersion) -> bool;
}
