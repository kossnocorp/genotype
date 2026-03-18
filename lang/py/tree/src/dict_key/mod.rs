use serde::Serialize;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PYDictKey {
    Int,
    Float,
    String,
    Boolean,
}
