use std::fs::read_to_string;

use genotype_parser::{
    tree::{GTModule, GTModuleParse},
    GTSourceCode,
};

use super::GTProjectModulePath;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleParse(pub GTProjectModulePath, pub GTModuleParse);

impl<'a> GTProjectModuleParse {
    pub fn try_new(path: GTProjectModulePath) -> Result<Self, Box<dyn std::error::Error>> {
        let code = read_to_string(&path)?;
        let source_code = GTSourceCode {
            name: path.as_name(),
            content: code.clone(),
        };
        let parse = GTModule::parse(source_code)?;
        Ok(Self(path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GTProjectModulePath>, Box<dyn std::error::Error>> {
        self.1
            .resolve
            .deps
            .iter()
            .map(|dep| self.0.resolve(dep))
            .collect()
    }
}
