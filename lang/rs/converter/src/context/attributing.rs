use genotype_lang_rs_tree::RSAttribute;

use super::RSConvertContext;

impl RSConvertContext {
    pub fn attribute_field(&mut self, attribute: String) {
        self.field_attributes.push(RSAttribute(attribute));
    }

    pub fn drain_field_attributes(&mut self) -> Vec<RSAttribute> {
        self.field_attributes.drain(..).collect()
    }
}
