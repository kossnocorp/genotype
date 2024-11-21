use crate::diagnostic::span::GTSpan;

use super::{GTDefinitionId, GTIdentifier};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTBranded {
    Boolean(GTSpan, GTDefinitionId, GTIdentifier),
    String(GTSpan, GTDefinitionId, GTIdentifier),
    Int(GTSpan, GTDefinitionId, GTIdentifier),
    Float(GTSpan, GTDefinitionId, GTIdentifier),
    Null(GTSpan, GTDefinitionId, GTIdentifier),
}
