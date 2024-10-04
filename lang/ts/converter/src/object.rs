use genotype_lang_ts_tree::{definition::TSDefinition, object::TSObject};
use genotype_parser::tree::object::GTObject;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSObject> for GTObject {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSObject
    where
        HoistFn: Fn(TSDefinition),
    {
        TSObject {
            properties: self
                .properties
                .iter()
                .map(|property| property.convert(resolve, hoist))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
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
                        descriptor: GTPrimitive::String.into(),
                        required: true,
                    },
                    GTProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: GTPrimitive::Int.into(),
                        required: false,
                    }
                ]
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
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
