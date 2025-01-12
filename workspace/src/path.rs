use std::path::PathBuf;

use miette::Result;

use crate::error::GTWError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GTWPath {
    path: PathBuf,
    kind: GTWPathKind,
}

impl GTWPath {
    pub fn new(path: &String) -> Result<GTWPath> {
        let absolute_path = PathBuf::from(path)
            .canonicalize()
            .map_err(|_| GTWError::CanonicalizePath(path.clone()))?;
        let kind = GTWPath::detect_kind(&absolute_path)?;
        Ok(GTWPath {
            path: absolute_path,
            kind,
        })
    }

    pub fn detect_kind(path: &PathBuf) -> Result<GTWPathKind> {
        let ext = path.extension().and_then(|ext| ext.to_str());
        match ext {
            Some("toml") => {
                if path.starts_with("genotype") {
                    return Ok(GTWPathKind::Config);
                }
            }
            Some("type") => {
                return Ok(GTWPathKind::Module);
            }
            _ => {}
        }
        Err(GTWError::DetectKind(path.display().to_string()).into())
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn kind(&self) -> &GTWPathKind {
        &self.kind
    }
}

impl From<GTWPath> for String {
    fn from(path: GTWPath) -> String {
        path.path.display().to_string()
    }
}

impl From<&GTWPath> for String {
    fn from(path: &GTWPath) -> String {
        path.path.display().to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum GTWPathKind {
    Config,
    Module,
}
