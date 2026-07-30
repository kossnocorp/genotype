use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsRecordKey {
    Reference(#[visit] TsReference),
    Number,
    String,
    Boolean,
}
