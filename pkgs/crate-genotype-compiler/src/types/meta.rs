use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaNew {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaLoadedProject {
    pub paths: GtcMetaLoadedProjectPaths,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaLoadedProjectPaths {
    pub src: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaLoadedModules {
    pub paths: GtcMetaLoadedProjectPaths,
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaCompiled {
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub paths: GtcMetaCompiledPaths,
    pub modules: Vec<GtcMetaCompiledModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaCompiledPaths {
    pub src: String,
    pub dist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<GtcMetaCompiledPathsLang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<GtcMetaCompiledPathsLang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<GtcMetaCompiledPathsLang>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaCompiledPathsLang {
    pub pkg: String,
    pub src: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtcMetaCompiledModule {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<String>,
}
