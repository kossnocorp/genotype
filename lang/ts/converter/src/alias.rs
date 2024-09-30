use genotype_lang_ts_tree::{
    alias::TSAlias, definition::TSDefinition, definition_descriptor::TSDefinitionDescriptor,
    primitive::TSPrimitive, type_descriptor::TSTypeDescriptor,
};
use genotype_parser::tree::alias::GTAlias;

use crate::convert::TSConvert;

impl TSConvert<TSDefinition> for GTAlias {
    fn convert(&self) -> TSDefinition {
        TSDefinition {
            doc: None,
            descriptor: TSDefinitionDescriptor::Alias(TSAlias {
                name: self.name.convert(),
                // [TODO]
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::name::TSName;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{
        alias::GTAlias, descriptor::GTDescriptor, name::GTName, primitive::GTPrimitive,
    };

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                doc: None,
                name: GTName("Name".to_string()),
                descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
            }
            .convert(),
            TSDefinition {
                doc: None,
                descriptor: TSDefinitionDescriptor::Alias(TSAlias {
                    name: TSName("Name".to_string()),
                    descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                }),
            }
        );
    }
}
