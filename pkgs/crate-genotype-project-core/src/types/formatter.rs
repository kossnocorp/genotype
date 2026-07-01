use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtpFormatter {
    Shell(GtpFormatterShell),
    Executor(GtpFormatterExecutor),
    PresetOxfmt(GtpFormatterPresetOxfmt),
    PresetPrettier(GtpFormatterPresetPrettier),
    PresetRuff(GtpFormatterPresetRuff),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpFormatterGenericBase {
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "shell")]
pub struct GtpFormatterShell {
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpFormatterExecutor {
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    pub kind: GtpFormatterExecutorKind,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtpFormatterExecutorKind {
    #[literal("cargo")]
    Cargo,
    Node(GtpFormatterExecutorKindNode),
    Python(GtpFormatterExecutorKindPython),
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtpFormatterExecutorKindNode {
    #[literal("pnpm")]
    Pnpm,
    #[literal("pnx")]
    Pnx,
    #[literal("npm")]
    Npm,
    #[literal("npx")]
    Npx,
    #[literal("bun")]
    Bun,
    #[literal("bunx")]
    Bunx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpFormatterPresetNodeBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<GtpFormatterExecutorKindNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "oxfmt")]
pub struct GtpFormatterPresetOxfmt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<GtpFormatterExecutorKindNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "prettier")]
pub struct GtpFormatterPresetPrettier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<GtpFormatterExecutorKindNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpFormatterPresetPythonBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<GtpFormatterExecutorKindPython>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtpFormatterExecutorKindPython {
    #[literal("uv")]
    Uv,
    #[literal("poetry")]
    Poetry,
    #[literal("pipx")]
    Pipx,
}

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[literals(kind = "ruff")]
pub struct GtpFormatterPresetRuff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<GtpFormatterExecutorKindPython>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpFormatterCmd {
    pub cmd: String,
    pub args: Vec<String>,
}
