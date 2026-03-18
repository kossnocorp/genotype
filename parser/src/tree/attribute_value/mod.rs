use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTAttributeValue {
    Literal(GTLiteral),
    Identifier(GTIdentifier),
}

impl From<GTLiteral> for GTAttributeValue {
    fn from(literal: GTLiteral) -> Self {
        Self::Literal(literal)
    }
}

impl From<GTIdentifier> for GTAttributeValue {
    fn from(identifier: GTIdentifier) -> Self {
        Self::Identifier(identifier)
    }
}
