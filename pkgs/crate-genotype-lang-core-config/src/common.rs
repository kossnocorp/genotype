use crate::GtlConfigPkgPathSetting;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtlConfigCommon<Dist: GtlConfigPkgPathSetting> {
    /// Whether to enable the target package generation.
    #[serde(default)]
    pub enabled: bool,
    /// Output directory.
    #[serde(default)]
    pub dist: Dist,
    /// Whether to generate target package files and package directory layout.
    pub package: Option<bool>,
    /// Manifest configuration.
    #[serde(default)]
    pub manifest: Table,
    /// Manually mapped dependencies.
    #[serde(default)]
    pub dependencies: IndexMap<String, String>,
}
