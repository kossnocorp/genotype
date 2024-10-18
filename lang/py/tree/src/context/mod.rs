use crate::{PYIdentifier, PYPath, PYVersion};

#[cfg(test)]
pub mod mock;

pub trait PYContext {
    fn import(&mut self, path: PYPath, name: PYIdentifier);

    fn is_version(&self, version: PYVersion) -> bool;
}
