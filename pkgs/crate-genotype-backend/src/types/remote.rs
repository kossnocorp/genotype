// Do not edit manually! Code generated from ../../types/remote.type

use genotype_core::GtDiagnostic;
use genotype_project_core::GtpFormatter;
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    RemoveFile(GtbRemoteBackendRequestRemoveFile),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    RemoveFile(GtbRemoteBackendRequestResponseRemoveFile),
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "glob-files")]
pub struct GtbRemoteBackendRequestGlobFiles {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "glob-files")]
pub struct GtbRemoteBackendRequestResponseGlobFiles {
    pub paths: Vec<String>,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "read-file")]
pub struct GtbRemoteBackendRequestReadFile {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "read-file")]
pub struct GtbRemoteBackendRequestResponseReadFile {
    pub content: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "file-exists")]
pub struct GtbRemoteBackendRequestFileExists {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "file-exists")]
pub struct GtbRemoteBackendRequestResponseFileExists {
    pub exists: bool,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "is-file")]
pub struct GtbRemoteBackendRequestIsFile {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "is-file")]
pub struct GtbRemoteBackendRequestResponseIsFile {
    #[serde(rename = "isFile")]
    pub is_file: bool,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "find-file")]
pub struct GtbRemoteBackendRequestFindFile {
    #[serde(rename = "fileName")]
    pub file_name: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "find-file")]
pub struct GtbRemoteBackendRequestResponseFindFile {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "report-diagnostic")]
pub struct GtbRemoteBackendRequestReportDiagnostic {
    pub diagnostic: GtDiagnostic,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "report-diagnostic")]
pub struct GtbRemoteBackendRequestResponseReportDiagnostic {}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "run-formatter")]
pub struct GtbRemoteBackendRequestRunFormatter {
    pub formatter: GtpFormatter,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "run-formatter")]
pub struct GtbRemoteBackendRequestResponseRunFormatter {}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "write-file")]
pub struct GtbRemoteBackendRequestWriteFile {
    pub path: String,
    pub content: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "write-file")]
pub struct GtbRemoteBackendRequestResponseWriteFile {}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "remove-file")]
pub struct GtbRemoteBackendRequestRemoveFile {
    pub path: String,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[literals(kind = "remove-file")]
pub struct GtbRemoteBackendRequestResponseRemoveFile {}
