use genotype_lang_py_config::PYVersion;

use crate::{PYDependency, PYIdentifier};

#[cfg(test)]
pub mod mock;

pub trait PYContext {
    fn import(&mut self, dependency: PYDependency, name: PYIdentifier);

    fn is_version(&self, version: PYVersion) -> bool;
}
