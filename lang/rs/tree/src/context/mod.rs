use genotype_lang_rs_config::RSVersion;

use crate::{RSDependency, RSIdentifier};

#[cfg(test)]
pub mod mock;

pub trait RSContext {
    fn import(&mut self, dependency: RSDependency, name: RSIdentifier);

    fn is_version(&self, version: RSVersion) -> bool;
}
