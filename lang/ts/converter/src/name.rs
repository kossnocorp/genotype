use genotype_lang_ts_tree::name::TSName;
use genotype_parser::tree::name::GTName;

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
    use genotype_parser::tree::name::GTName;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTName("Name".to_string()).convert(),
            TSName("Name".to_string())
        );
    }
}
