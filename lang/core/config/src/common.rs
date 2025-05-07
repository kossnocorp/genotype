use crate::GtlConfigPackagePathSetting;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Table;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtlConfigCommon<Out: GtlConfigPackagePathSetting> {
    /// Whether to enable the target package generation.
    #[serde(default)]
    pub enabled: bool,
    /// Output directory.
    #[serde(default)]
    pub out: Out,
    /// Manifest configuration.
    #[serde(default)]
    pub manifest: Table,
    /// Manually mapped dependencies.
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}
