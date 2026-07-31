// Do not edit manually! Code generated from ../../types/source_code.type

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtpSourceCode {
    pub content: String,
    pub hash: GtpSourceCodeHash,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct GtpSourceCodeHash(pub String);
