use serde::Serialize;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TSRecordKey {
    Number,
    String,
    Boolean,
}
