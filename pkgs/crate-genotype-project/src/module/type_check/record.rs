use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleTypeCheckRecordKeyResolve {
    pub primitive: GtPrimitiveKind,
    pub branded: bool,
}
