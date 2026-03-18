use serde::Serialize;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYAny;
