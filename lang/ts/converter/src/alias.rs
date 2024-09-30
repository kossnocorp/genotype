use genotype_lang_ts_tree::{alias::TSAlias, definition::TSDefinition, interface::TSInterface};
use genotype_parser::tree::{alias::GTAlias, descriptor::GTDescriptor};

use crate::convert::TSConvert;

impl TSConvert<TSDefinition> for GTAlias {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSDefinition
    where
        HoistFn: Fn(TSDefinition),
    {
        match &self.descriptor {
            GTDescriptor::Object(object) => TSDefinition::Interface(TSInterface {
                name: self.name.convert(hoist),
                properties: object.properties.iter().map(|p| p.convert(hoist)).collect(),
            }),

            _ => TSDefinition::Alias(TSAlias {
                name: self.name.convert(hoist),
                descriptor: self.descriptor.convert(hoist),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{
        interface::TSInterface, name::TSName, primitive::TSPrimitive, property::TSProperty,
        type_descriptor::TSTypeDescriptor,
    };
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{
        alias::GTAlias, descriptor::GTDescriptor, name::GTName, object::GTObject,
        primitive::GTPrimitive, property::GTProperty,
    };

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: GTName("Name".to_string()),
                descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
            }
            .convert(&|_| {}),
            TSDefinition::Alias(TSAlias {
                name: TSName("Name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: GTName("Book".to_string()),
                descriptor: GTDescriptor::Object(GTObject {
                    properties: vec![
                        GTProperty {
                            doc: None,
                            name: GTName("title".to_string()),
                            descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                            required: true,
                        },
                        GTProperty {
                            doc: None,
                            name: GTName("author".to_string()),
                            descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&|_| {}),
            TSDefinition::Interface(TSInterface {
                name: TSName("Book".to_string()),
                properties: vec![
                    TSProperty {
                        name: TSName("title".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: TSName("author".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }
}
