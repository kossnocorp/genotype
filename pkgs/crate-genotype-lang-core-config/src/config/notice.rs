use crate::prelude::internal::*;

#[derive(Serialize, Deserialize)]
pub struct GtlConfigNotice {
    pub kind: GtlConfigNoticeKind,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub enum GtlConfigNoticeKind {
    Warning,
    Info,
}
