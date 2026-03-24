use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttribute {
    pub span: GTSpan,
    #[visit]
    pub name: GTAttributeName,
    #[visit]
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
        self.name.value.as_ref() == name
    }

    pub fn get_assigned(&self, name: &str) -> Option<&GTAttributeAssignment> {
        if self.is_it(name) {
            if let Some(GTAttributeDescriptor::Assignment(assignment)) = &self.descriptor {
                return Some(&assignment);
            }
        }
        None
    }

    pub fn find_property(&self, name: &str) -> Option<String> {
        match &self.descriptor {
            Some(GTAttributeDescriptor::Assignment(assignment)) => {
                if self.name.value.as_ref() == name {
                    if let GTAttributeValue::Literal(literal) = &assignment.value {
                        if let GTLiteralValue::String(string) = &literal.value {
                            return Some(string.clone());
                        }
                    }
                }
            }
            Some(GTAttributeDescriptor::Properties(properties)) => {
                for property in properties.iter() {
                    if property.name.value.as_ref() == name {
                        if let GTAttributeValue::Literal(literal) = &property.value {
                            if let GTLiteralValue::String(string) = &literal.value {
                                return Some(string.clone());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub fn find_property_in(attributes: &Vec<GTAttribute>, name: &str) -> Option<String> {
        for attr in attributes.iter() {
            if let Some(value) = attr.find_property(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn find_flag(attributes: &Vec<GTAttribute>, name: &str) -> bool {
        attributes
            .iter()
            .any(|attr| attr.is_it(name) && attr.descriptor.is_none())
    }
}
