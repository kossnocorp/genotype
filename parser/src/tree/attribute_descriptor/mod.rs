use super::{GTAttributeAssignment, GTAttributeProperty, GTAttributeValue};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTAttributeDescriptor {
    Assigment(GTAttributeAssignment),
    Arguments(Vec<GTAttributeValue>),
    Properties(Vec<GTAttributeProperty>),
}
