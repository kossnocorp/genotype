// Do not edit manually! Code generated from ../../types/meta.type

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaNew {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaLoadedProject {
    pub paths: GtcMetaLoadedProjectPaths,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaLoadedProjectPaths {
    pub src: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaLoadedModules {
    pub paths: GtcMetaLoadedProjectPaths,
    pub modules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaCompiled {
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub paths: GtcMetaCompiledPaths,
    pub modules: Vec<GtcMetaCompiledModule>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaCompiledPathsLang {
    pub pkg: String,
    pub src: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtcMetaCompiledModule {
    pub src: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<String>,
}
