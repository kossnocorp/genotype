use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GTAttributeValue {
    Literal(#[visit] GTLiteral),
    Identifier(#[visit] GTIdentifier),
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
