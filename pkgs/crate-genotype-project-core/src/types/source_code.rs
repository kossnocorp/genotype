// Do not edit manually! Code generated from ../../types/source_code.type

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GtpSourceCode {
    pub content: String,
    pub hash: GtpSourceCodeHash,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GtpSourceCodeHash(pub String);
