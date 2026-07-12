use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtMeta {
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub paths: GtMetaPaths,
    pub timing: GtMetaTiming,
    pub modules: Vec<GtMetaModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtMetaPaths {
    pub src: String,
    pub dist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<GtMetaPathsLang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<GtMetaPathsLang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<GtMetaPathsLang>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtMetaPathsLang {
    pub pkg: String,
    pub src: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtMetaTiming {
    #[serde(rename = "totalMs")]
    pub total_ms: u64,
    #[serde(rename = "loadProjectMs")]
    pub load_project_ms: u64,
    #[serde(rename = "loadModulesMs")]
    pub load_modules_ms: u64,
    #[serde(rename = "compileMs")]
    pub compile_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtMetaModule {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<String>,
}
