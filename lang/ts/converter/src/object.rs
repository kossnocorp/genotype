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
                span: (0, 0).into(),
                name: GTIdentifier::new((0, 0).into(), "Person".into()).into(),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSObject {
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSUnion {
                            descriptors: vec![
                                TSPrimitive::Number.into(),
                                TSPrimitive::Undefined.into()
                            ]
                        }
                        .into(),
                        required: false,
                    }
                ]
            }
        );
    }
}
