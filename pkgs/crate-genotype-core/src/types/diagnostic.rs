// Do not edit manually! Code generated from ../../types/diagnostic.type

use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtDiagnostic {
    pub kind: GtDiagnosticKind,
    pub content: GtDiagnosticContent,
}

#[serde_literals]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GtDiagnosticContent {
    Message(GtDiagnosticContentMessage),
    Report(GtDiagnosticContentReport),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtDiagnosticContentBase {
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtDiagnosticContentMessage {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<GtDiagnosticContentMessageBody>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtDiagnosticContentReport {
    pub title: String,
    pub report: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GtDiagnosticContentMessageBody {
    Single(GtDiagnosticContentMessageBodySingle),
    Multi(GtDiagnosticContentMessageBodyMulti),
}

pub type GtDiagnosticContentMessageBodySingle = String;

pub type GtDiagnosticContentMessageBodyMulti = Vec<String>;
