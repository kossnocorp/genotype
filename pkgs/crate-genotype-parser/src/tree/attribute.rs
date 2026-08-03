use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttribute {
    pub span: GtSpan,
    #[visit]
    pub name: GtAttributeName,
    #[visit]
    pub descriptor: Option<GtAttributeDescriptor>,
}

impl GtAttribute {
    pub fn new(
        span: GtSpan,
        name: GtAttributeName,
        descriptor: Option<GtAttributeDescriptor>,
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

    pub fn get_assigned(&self, name: &str) -> Option<&GtAttributeAssignment> {
        if self.is_it(name)
            && let Some(GtAttributeDescriptor::Assignment(assignment)) = &self.descriptor
        {
            return Some(assignment);
        }
        None
    }

    pub fn find_property(&self, name: &str) -> Option<String> {
        match &self.descriptor {
            Some(GtAttributeDescriptor::Assignment(assignment)) => {
                if self.is_it(name)
                    && let GtAttributeValue::Literal(literal) = &assignment.value
                    && let GtLiteralValue::String(string) = &literal.value
                {
                    return Some(string.clone());
                }
            }
            Some(GtAttributeDescriptor::Properties(properties)) => {
                for property in properties {
                    if property.name.value.as_ref() == name
                        && let GtAttributeValue::Literal(literal) = &property.value
                        && let GtLiteralValue::String(string) = &literal.value
                    {
                        return Some(string.clone());
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub fn find_property_in(attributes: &[GtAttribute], name: &str) -> Option<String> {
        attributes
            .iter()
            .find_map(|attribute| attribute.find_property(name))
    }

    pub fn find_flag(attributes: &[GtAttribute], name: &str) -> bool {
        attributes
            .iter()
            .any(|attribute| attribute.is_it(name) && attribute.descriptor.is_none())
    }
}
