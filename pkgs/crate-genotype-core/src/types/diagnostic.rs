use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtDiagnostic {
    pub kind: GtDiagnosticKind,
    pub content: GtDiagnosticContent,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtDiagnosticKind {
    #[literal("error")]
    Error,
    #[literal("warning")]
    Warning,
    #[literal("success")]
    Success,
    #[literal("info")]
    Info,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtDiagnosticContent {
    Message(GtDiagnosticContentMessage),
    Report(GtDiagnosticContentReport),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtDiagnosticContentBase {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtDiagnosticContentMessage {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<GtDiagnosticContentMessageBody>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtDiagnosticContentReport {
    pub title: String,
    pub report: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtDiagnosticContentMessageBody {
    Single(GtDiagnosticContentMessageBodySingle),
    Multi(GtDiagnosticContentMessageBodyMulti),
}

pub type GtDiagnosticContentMessageBodySingle = String;

pub type GtDiagnosticContentMessageBodyMulti = Vec<String>;
