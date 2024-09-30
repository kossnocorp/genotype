use genotype_lang_ts_tree::{
    primitive::TSPrimitive, property::TSProperty, type_descriptor::TSTypeDescriptor,
};
use genotype_parser::tree::property::GTProperty;

use crate::convert::TSConvert;

impl TSConvert<TSProperty> for GTProperty {
    fn convert(&self) -> TSProperty {
        TSProperty {
            name: self.name.convert(),
            // [TODO]
            // descriptor: self.descriptor.convert(),
            descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
            required: self.required,
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::name::TSName;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{descriptor::GTDescriptor, name::GTName, primitive::GTPrimitive};

    #[test]
    fn test_convert() {
        assert_eq!(
            GTProperty {
                doc: None,
                name: GTName("name".to_string()),
                descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                required: false,
            }
            .convert(),
            TSProperty {
                name: TSName("name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                required: false,
            }
        );
    }
}
