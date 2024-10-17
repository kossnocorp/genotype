use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GTObjectName {
    Named(GTIdentifier),
    Anonymous(GTSpan, GTObjectNameParent),
    Alias(GTIdentifier),
}

impl From<GTIdentifier> for GTObjectName {
    fn from(value: GTIdentifier) -> Self {
        GTObjectName::Named(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GTObjectNameParent {
    Alias(GTIdentifier),
    Key(GTIdentifier, Vec<GTKey>),
}
