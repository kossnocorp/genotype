use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtAttributeValue {
    Literal(#[visit] GtLiteral),
    Identifier(#[visit] GtIdentifier),
}

impl From<GtLiteral> for GtAttributeValue {
    fn from(literal: GtLiteral) -> Self {
        Self::Literal(literal)
    }
}

impl From<GtIdentifier> for GtAttributeValue {
    fn from(identifier: GtIdentifier) -> Self {
        Self::Identifier(identifier)
    }
}
