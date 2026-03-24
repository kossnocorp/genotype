use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]

pub enum PYLiteral {
    None,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
