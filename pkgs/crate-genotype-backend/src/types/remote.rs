use genotype_core::{GtDiagnostic, GtMeta};
use genotype_project_core::GtpFormatterCmd;
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtbRemoteBackendRequest {
    GlobFiles(GtbRemoteBackendRequestGlobFiles),
    ReadFile(GtbRemoteBackendRequestReadFile),
    FileExists(GtbRemoteBackendRequestFileExists),
    IsFile(GtbRemoteBackendRequestIsFile),
    FindFile(GtbRemoteBackendRequestFindFile),
    ReportDiagnostic(GtbRemoteBackendRequestReportDiagnostic),
    RunFormatter(GtbRemoteBackendRequestRunFormatter),
    WriteFile(GtbRemoteBackendRequestWriteFile),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtbRemoteBackendRequestResponse {
    GlobFiles(GtbRemoteBackendRequestResponseGlobFiles),
    ReadFile(GtbRemoteBackendRequestResponseReadFile),
    FileExists(GtbRemoteBackendRequestResponseFileExists),
    IsFile(GtbRemoteBackendRequestResponseIsFile),
    FindFile(GtbRemoteBackendRequestResponseFindFile),
    ReportDiagnostic(GtbRemoteBackendRequestResponseReportDiagnostic),
    RunFormatter(GtbRemoteBackendRequestResponseRunFormatter),
    WriteFile(GtbRemoteBackendRequestResponseWriteFile),
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "glob-files")]
pub struct GtbRemoteBackendRequestGlobFiles {
    pub path: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "glob-files")]
pub struct GtbRemoteBackendRequestResponseGlobFiles {
    pub paths: Vec<String>,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "read-file")]
pub struct GtbRemoteBackendRequestReadFile {
    pub path: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "read-file")]
pub struct GtbRemoteBackendRequestResponseReadFile {
    pub content: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "file-exists")]
pub struct GtbRemoteBackendRequestFileExists {
    pub path: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "file-exists")]
pub struct GtbRemoteBackendRequestResponseFileExists {
    pub exists: bool,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "is-file")]
pub struct GtbRemoteBackendRequestIsFile {
    pub path: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "is-file")]
pub struct GtbRemoteBackendRequestResponseIsFile {
    #[serde(rename = "isFile")]
    pub is_file: bool,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "find-file")]
pub struct GtbRemoteBackendRequestFindFile {
    #[serde(rename = "fileName")]
    pub file_name: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "find-file")]
pub struct GtbRemoteBackendRequestResponseFindFile {
    pub path: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "report-diagnostic")]
pub struct GtbRemoteBackendRequestReportDiagnostic {
    pub diagnostic: GtDiagnostic,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "report-diagnostic")]
pub struct GtbRemoteBackendRequestResponseReportDiagnostic {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "run-formatter")]
pub struct GtbRemoteBackendRequestRunFormatter {
    pub cmd: GtpFormatterCmd,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "run-formatter")]
pub struct GtbRemoteBackendRequestResponseRunFormatter {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "write-file")]
pub struct GtbRemoteBackendRequestWriteFile {
    pub path: String,
    pub content: String,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "write-file")]
pub struct GtbRemoteBackendRequestResponseWriteFile {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtbRemoteRuntimeRequest {
    LoadInProject(GtbRemoteRuntimeRequestLoadInProject),
    LoadInModules(GtbRemoteRuntimeRequestLoadInModules),
    Compile(GtbRemoteRuntimeRequestCompile),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtbRemoteRuntimeRequestResponse {
    LoadInProject(GtbRemoteRuntimeRequestResponseLoadInProject),
    LoadInModules(GtbRemoteRuntimeRequestResponseLoadInModules),
    Compile(GtbRemoteRuntimeRequestResponseCompile),
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-project")]
pub struct GtbRemoteRuntimeRequestLoadInProject {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-project")]
pub struct GtbRemoteRuntimeRequestResponseLoadInProject {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-modules")]
pub struct GtbRemoteRuntimeRequestLoadInModules {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "load-in-modules")]
pub struct GtbRemoteRuntimeRequestResponseLoadInModules {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "compile")]
pub struct GtbRemoteRuntimeRequestCompile {}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "compile")]
pub struct GtbRemoteRuntimeRequestResponseCompile {
    pub meta: GtMeta,
}
