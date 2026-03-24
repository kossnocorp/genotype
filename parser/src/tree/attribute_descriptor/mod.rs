use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GTAttributeDescriptor {
    Assignment(#[visit] GTAttributeAssignment),
    Arguments(Vec<GTAttributeValue>),
    Properties(#[visit] Vec<GTAttributeProperty>),
}

impl From<GTAttributeAssignment> for GTAttributeDescriptor {
    fn from(value: GTAttributeAssignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<Vec<GTAttributeValue>> for GTAttributeDescriptor {
    fn from(value: Vec<GTAttributeValue>) -> Self {
        Self::Arguments(value)
    }
}

impl From<Vec<GTAttributeProperty>> for GTAttributeDescriptor {
    fn from(value: Vec<GTAttributeProperty>) -> Self {
        Self::Properties(value)
    }
}
