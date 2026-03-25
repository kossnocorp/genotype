use crate::prelude::internal::*;

use super::RsConvertContext;

impl RsConvertContext {
    pub fn attribute_field(&mut self, attribute: String) {
        self.field_attributes.push(RsAttribute(attribute));
    }

    pub fn drain_field_attributes(&mut self) -> Vec<RsAttribute> {
        self.field_attributes.drain(..).collect()
    }
}
