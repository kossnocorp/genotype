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

impl GTBranded {
    pub fn name(&self) -> GTIdentifier {
        match self {
            GTBranded::Boolean(_, _, identifier) => identifier.clone(),
            GTBranded::String(_, _, identifier) => identifier.clone(),
            GTBranded::Int(_, _, identifier) => identifier.clone(),
            GTBranded::Float(_, _, identifier) => identifier.clone(),
            GTBranded::Null(_, _, identifier) => identifier.clone(),
        }
    }

    pub fn span(&self) -> GTSpan {
        match self {
            GTBranded::Boolean(span, _, _) => span.clone(),
            GTBranded::String(span, _, _) => span.clone(),
            GTBranded::Int(span, _, _) => span.clone(),
            GTBranded::Float(span, _, _) => span.clone(),
            GTBranded::Null(span, _, _) => span.clone(),
        }
    }
}
