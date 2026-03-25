use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsLiteral {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
