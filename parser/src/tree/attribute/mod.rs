use crate::GTSpan;

use super::{GTAttributeAssignment, GTAttributeDescriptor, GTAttributeName};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttribute {
    pub span: GTSpan,
    pub name: GTAttributeName,
    pub descriptor: Option<GTAttributeDescriptor>,
}

impl GTAttribute {
    pub fn new(
        span: GTSpan,
        name: GTAttributeName,
        descriptor: Option<GTAttributeDescriptor>,
    ) -> Self {
        Self {
            span,
            name,
            descriptor,
        }
    }

    pub fn is_it(&self, name: &str) -> bool {
        self.name.name == name
    }

    pub fn get_assigned(&self, name: &str) -> Option<&GTAttributeAssignment> {
        if self.is_it(name) {
            if let Some(GTAttributeDescriptor::Assignment(assignment)) = &self.descriptor {
                return Some(&assignment);
            }
        }
        None
    }
}
