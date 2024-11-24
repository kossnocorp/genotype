use std::path::PathBuf;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};

use crate::{error::GTConfigError, result::GTConfigResult, GTConfig};

pub const GTCONFIG_FILE: &str = "genotype.toml";

impl GTConfig {
    pub fn load(path: &PathBuf) -> GTConfigResult<Self> {
        let file = Self::find(path)?;

        let mut config: GTConfig = Figment::from(Serialized::defaults(GTConfig::default()))
            // [TODO] Integrate with CLI:
            // .merge(Serialized::defaults(GTConfig::parse()))
            .merge(Toml::file(file))
            .merge(Env::prefixed("GT_"))
            .extract()?;

        if let None = config.root {
            config.root = Some(path.to_path_buf());
        }

        Ok(config)
    }

    fn find(path: &PathBuf) -> GTConfigResult<PathBuf> {
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

        Err(GTConfigError::MissingConfig(path.clone()))
    }
}
