use genotype_lang_ts_tree::{alias::TSAlias, definition::TSDefinition, interface::TSInterface};
use genotype_parser::tree::{alias::GTAlias, descriptor::GTDescriptor};

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSDefinition> for GTAlias {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSDefinition
    where
        HoistFn: Fn(TSDefinition),
    {
        match &self.descriptor {
            GTDescriptor::Object(object) => TSDefinition::Interface(TSInterface {
                name: self.name.convert(resolve, hoist),
                extensions: vec![],
                properties: object
                    .properties
                    .iter()
                    .map(|p| p.convert(resolve, hoist))
                    .collect(),
            }),

            _ => TSDefinition::Alias(TSAlias {
                name: self.name.convert(resolve, hoist),
                descriptor: self.descriptor.convert(resolve, hoist),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: "Name".into(),
                descriptor: GTPrimitive::Boolean.into(),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
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
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            doc: None,
                            name: "title".into(),
                            descriptor: GTPrimitive::String.into(),
                            required: true,
                        },
                        GTProperty {
                            doc: None,
                            name: "author".into(),
                            descriptor: GTPrimitive::String.into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDefinition::Interface(TSInterface {
                name: "Book".into(),
                extensions: vec![],
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
