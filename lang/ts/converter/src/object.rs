use genotype_lang_ts_tree::{
    alias::TSAlias, definition::TSDefinition, definition_descriptor::TSDefinitionDescriptor,
    name::TSName, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor,
};
use genotype_parser::tree::{alias::GTAlias, name::GTName};

use crate::convert::TSConvert;

impl TSConvert<TSName> for GTName {
    fn convert(&self) -> TSName {
        TSName(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{
        alias::GTAlias, descriptor::GTDescriptor, name::GTName, primitive::GTPrimitive,
    };

    #[test]
    fn test_convert() {
        assert_eq!(
            GTName("Name".to_string()).convert(),
            TSName("Name".to_string())
        );
    }
}
