// Do not edit manually! Code generated from ../../types/lang.type

use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[serde_literals]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtpLang {
    #[literal("py")]
    Py,
    #[literal("rs")]
    Rs,
    #[literal("ts")]
    Ts,
}
