use serde::{Deserialize, Serialize};

/// Module identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtModuleId(pub String);
