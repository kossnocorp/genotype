use serde::Serialize;

use crate::diagnostic::span::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTAny(pub GTSpan);
