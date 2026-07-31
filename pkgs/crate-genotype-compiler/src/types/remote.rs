// Do not edit manually! Code generated from ../../types/remote.type

use super::meta::{GtcMetaCompiled, GtcMetaLoadedModules, GtcMetaLoadedProject};
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtcRemoteRuntimeRequest {
    LoadInProject(GtcRemoteRuntimeRequestLoadInProject),
    LoadInModules(GtcRemoteRuntimeRequestLoadInModules),
    Compile(GtcRemoteRuntimeRequestCompile),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtcRemoteRuntimeRequestResponse {
    LoadInProject(GtcRemoteRuntimeRequestResponseLoadInProject),
    LoadInModules(GtcRemoteRuntimeRequestResponseLoadInModules),
    Compile(GtcRemoteRuntimeRequestResponseCompile),
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-project")]
pub struct GtcRemoteRuntimeRequestLoadInProject {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-project")]
pub struct GtcRemoteRuntimeRequestResponseLoadInProject {
    pub meta: GtcMetaLoadedProject,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-modules")]
pub struct GtcRemoteRuntimeRequestLoadInModules {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-modules")]
pub struct GtcRemoteRuntimeRequestResponseLoadInModules {
    pub meta: GtcMetaLoadedModules,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "compile")]
pub struct GtcRemoteRuntimeRequestCompile {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "compile")]
pub struct GtcRemoteRuntimeRequestResponseCompile {
    pub meta: GtcMetaCompiled,
}
