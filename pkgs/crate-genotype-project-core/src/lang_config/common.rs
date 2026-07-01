use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtpLangConfigCommon {
    /// Whether to enable the target package generation.
    #[serde(default)]
    pub enabled: bool,
    /// Output directory.
    #[serde(default)]
    pub dist: Option<GtpDistDirRelativePkgDirPath>,
    /// Whether to generate target package files and package directory layout.
    pub package: Option<bool>,
    /// Manifest configuration.
    #[serde(default)]
    pub manifest: Table,
    /// Manually mapped dependencies.
    #[serde(default)]
    pub dependencies: IndexMap<String, String>,
    /// Target-specific formatters to run after target compilation.
    #[serde(default)]
    pub formatters: Vec<GtpFormatter>,
}
