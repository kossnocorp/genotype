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
        descriptor::TSDescriptor, primitive::TSPrimitive, property::TSProperty,
    };
    use genotype_parser::tree::{
        descriptor::GTDescriptor, primitive::GTPrimitive, property::GTProperty,
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
                        name: "name".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                        required: true,
                    },
                    GTProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                        required: false,
                    }
                ]
            }
            .convert(&|_| {}),
            TSObject {
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false,
                    }
                ]
            }
        );
    }
}
