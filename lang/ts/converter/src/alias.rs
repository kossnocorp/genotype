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
        descriptor::TSDescriptor, interface::TSInterface, primitive::TSPrimitive,
        property::TSProperty, reference::TSReference,
    };
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{
        alias::GTAlias, descriptor::GTDescriptor, object::GTObject, primitive::GTPrimitive,
        property::GTProperty, reference::GTReference,
    };

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: "Name".into(),
                descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
            }
            .convert(&|_| {}),
            TSDefinition::Alias(TSAlias {
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: "Book".into(),
                descriptor: GTDescriptor::Object(GTObject {
                    properties: vec![
                        GTProperty {
                            doc: None,
                            name: "title".into(),
                            descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                            required: true,
                        },
                        GTProperty {
                            doc: None,
                            name: "author".into(),
                            descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&|_| {}),
            TSDefinition::Interface(TSInterface {
                name: "Book".into(),
                properties: vec![
                    TSProperty {
                        name: "title".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: "author".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }
}
