use miette::Result;

use crate::{error::GTWError, path::GTWPath};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum GTWFileKind {
    Config,
    Module,
}

impl GTWFileKind {
    pub fn detect(path: &GTWPath) -> Result<Self> {
        let path = path.as_path();
        let ext = path.extension().and_then(|ext| ext.to_str());
        match ext {
            Some("toml") => {
                if path.starts_with("genotype") {
                    return Ok(GTWFileKind::Config);
                }
            }
            Some("type") => {
                return Ok(GTWFileKind::Module);
            }
            _ => {}
        }
        Err(GTWError::DetectKind(path.display().to_string()).into())
    }
}
