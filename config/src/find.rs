use std::path::PathBuf;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use miette::{IntoDiagnostic, Result};

use crate::{error::GtConfigError, GtConfig};

pub const GTCONFIG_FILE: &str = "genotype.toml";

impl GtConfig {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let file = Self::find(path)?;

        let config: GtConfig = Figment::from(Serialized::defaults(GtConfig::default()))
            // [TODO] Integrate with CLI:
            // .merge(Serialized::defaults(GTConfig::parse()))
            .merge(Toml::file(file))
            .merge(Env::prefixed("GT_"))
            .extract()
            .into_diagnostic()?;

        Ok(config)
    }

    fn find(path: &PathBuf) -> Result<PathBuf> {
        let mut current = if path.is_dir() {
            Some(path.as_path())
        } else {
            path.parent()
        };

        while let Some(dir) = current {
            let file = dir.join(GTCONFIG_FILE);
            if file.is_file() {
                return Ok(file);
            }
            current = dir.parent();
        }

        Err(GtConfigError::MissingConfig(path.clone())).into_diagnostic()
    }
}
