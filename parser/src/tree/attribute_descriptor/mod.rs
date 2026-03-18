use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTAttributeDescriptor {
    Assignment(GTAttributeAssignment),
    Arguments(Vec<GTAttributeValue>),
    Properties(Vec<GTAttributeProperty>),
}
