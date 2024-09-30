use genotype_lang_ts_tree::{definition::TSDefinition, object::TSObject};
use genotype_parser::tree::object::GTObject;

use crate::convert::TSConvert;

impl TSConvert<TSObject> for GTObject {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSObject
    where
        HoistFn: Fn(TSDefinition),
    {
        TSObject {
            properties: self
                .properties
                .iter()
                .map(|property| property.convert(hoist))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{
        name::TSName, primitive::TSPrimitive, property::TSProperty,
        type_descriptor::TSTypeDescriptor,
    };
    use genotype_parser::tree::{
        descriptor::GTDescriptor, name::GTName, primitive::GTPrimitive, property::GTProperty,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTObject {
                properties: vec![
                    GTProperty {
                        doc: None,
                        name: GTName("name".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                        required: true,
                    },
                    GTProperty {
                        doc: None,
                        name: GTName("age".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                        required: false,
                    }
                ]
            }
            .convert(&|_| {}),
            TSObject {
                properties: vec![
                    TSProperty {
                        name: TSName("name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        required: false,
                    }
                ]
            }
        );
    }
}
