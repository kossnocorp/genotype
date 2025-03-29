use std::fs::read_to_string;

use genotype_parser::{GTModule, GTModuleId, GTModuleParse};
use miette::{NamedSource, Result};

use crate::error::GTProjectError;

use super::GTPModulePath;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleParse(pub GTPModulePath, pub GTModuleParse);

impl<'a> GTProjectModuleParse {
    pub fn try_new(id: GTModuleId, path: GTPModulePath) -> Result<Self> {
        let code = read_to_string(&path).map_err(|_| {
            GTProjectError::NotFound(path.as_path().as_os_str().to_str().unwrap().to_owned())
        })?;

        let source_code = NamedSource::new(path.as_path_str(), code.clone());
        let parse = GTModule::parse(id, source_code)?;
        Ok(Self(path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GTPModulePath>> {
        self.1
            .resolve
            .deps
            .iter()
            .map(|dep| self.0.resolve(dep))
            .collect()
    }
}
