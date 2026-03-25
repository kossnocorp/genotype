use miette::Result;

use crate::{error::GtwError, path::GtwPath};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum GtwFileKind {
    Config,
    Module,
}

impl GtwFileKind {
    pub fn detect(path: &GtwPath) -> Result<Self> {
        let path = path.as_path();
        let ext = path.extension().and_then(|ext| ext.to_str());
        match ext {
            Some("toml") => {
                if path.starts_with("genotype") {
                    return Ok(GtwFileKind::Config);
                }
            }
            Some("type") => {
                return Ok(GtwFileKind::Module);
            }
            _ => {}
        }
        Err(GtwError::DetectKind(path.display().to_string()).into())
    }
}
