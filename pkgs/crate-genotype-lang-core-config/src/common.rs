use crate::GtlConfigPkgPathSetting;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Table;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtlConfigCommon<Dist: GtlConfigPkgPathSetting> {
    /// Whether to enable the target package generation.
    #[serde(default)]
    pub enabled: bool,
    /// Output directory.
    #[serde(default, alias = "out")]
    pub dist: Dist,
    /// Manifest configuration.
    #[serde(default)]
    pub manifest: Table,
    /// Manually mapped dependencies.
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}
