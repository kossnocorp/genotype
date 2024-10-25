use crate::diagnostic::span::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAny(pub GTSpan);
