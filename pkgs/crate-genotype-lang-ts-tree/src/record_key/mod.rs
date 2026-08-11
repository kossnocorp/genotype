use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsRecordKey {
    Reference(#[visit] TsReference),
    BooleanReference(#[visit] TsReference, bool),
    Number,
    String,
    Boolean,
}
