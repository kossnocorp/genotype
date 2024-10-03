use genotype_lang_ts_tree::{definition::TSDefinition, property::TSProperty};
use genotype_parser::tree::property::GTProperty;

use crate::convert::TSConvert;

impl TSConvert<TSProperty> for GTProperty {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSProperty
    where
        HoistFn: Fn(TSDefinition),
    {
        TSProperty {
            name: self.name.convert(hoist),
            descriptor: self.descriptor.convert(hoist),
            required: self.required,
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
    fn test_convert() {
        assert_eq!(
            GTProperty {
                doc: None,
                name: "name".into(),
                descriptor: GTPrimitive::String.into(),
                required: false,
            }
            .convert(&|_| {}),
            TSProperty {
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: false,
            }
        );
    }
}
