use std::fs::read_to_string;

use genotype_parser::{
    tree::{GTModule, GTModuleParse},
    GTSourceCode,
};
use miette::Result;

use crate::error::GTProjectError;

use super::GTProjectModulePath;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleParse(pub GTProjectModulePath, pub GTModuleParse);

impl<'a> GTProjectModuleParse {
    pub fn try_new(path: GTProjectModulePath) -> Result<Self> {
        let code = read_to_string(&path).map_err(|_| {
            GTProjectError::NotFound(path.as_path().as_os_str().to_str().unwrap().to_owned())
        })?;

        let source_code = GTSourceCode {
            name: path.as_name(),
            content: code.clone(),
        };
        let parse = GTModule::parse(source_code)?;
        Ok(Self(path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GTProjectModulePath>> {
        self.1
            .resolve
            .deps
            .iter()
            .map(|dep| self.0.resolve(dep))
            .collect()
    }
}
