// Do not edit manually! Code generated from ../../types/build_info.type

use super::source_code::GtpSourceCodeHash;
use genotype_core::GtModuleId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpBuildInfo {
    pub src: GtpBuildInfoSrc,
    pub dist: GtpBuildInfoDist,
}

/// Path relative to the config dir
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct GtpBuildInfoPath(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpBuildInfoSrc {
    pub config_hash: GtpSourceCodeHash,
    pub modules: GtpBuildInfoSrcModules,
}

pub type GtpBuildInfoSrcModules = BTreeMap<GtpBuildInfoPath, GtpBuildInfoSrcModule>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpBuildInfoSrcModule {
    pub id: GtModuleId,
    pub hash: GtpSourceCodeHash,
    pub deps: Vec<GtModuleId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpBuildInfoDist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<GtpBuildInfoDistFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rs: Option<GtpBuildInfoDistFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub py: Option<GtpBuildInfoDistFiles>,
}

pub type GtpBuildInfoDistFiles = BTreeMap<GtpBuildInfoPath, GtpBuildInfoDistFile>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpBuildInfoDistFile {
    pub hash: GtpSourceCodeHash,
    #[serde(rename = "srcId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_id: Option<GtModuleId>,
}
