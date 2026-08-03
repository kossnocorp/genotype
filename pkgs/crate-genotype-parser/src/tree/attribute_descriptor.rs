use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtAttributeDescriptor {
    Assignment(#[visit] GtAttributeAssignment),
    Arguments(Vec<GtAttributeValue>),
    Properties(#[visit] Vec<GtAttributeProperty>),
}

impl From<GtAttributeAssignment> for GtAttributeDescriptor {
    fn from(value: GtAttributeAssignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<Vec<GtAttributeValue>> for GtAttributeDescriptor {
    fn from(value: Vec<GtAttributeValue>) -> Self {
        Self::Arguments(value)
    }
}

impl From<Vec<GtAttributeProperty>> for GtAttributeDescriptor {
    fn from(value: Vec<GtAttributeProperty>) -> Self {
        Self::Properties(value)
    }
}
