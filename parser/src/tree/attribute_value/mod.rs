use super::{GTIdentifier, GTLiteral};

mod parse;

#[derive(Debug, PartialEq, Clone)]
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
