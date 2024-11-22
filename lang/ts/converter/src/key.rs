use genotype_lang_ts_tree::key::TSKey;
use genotype_parser::tree::key::GTKey;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSKey> for GTKey {
    fn convert(&self, _context: &mut TSConvertContext) -> TSKey {
        TSKey(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSKey("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut Default::default()),
        );
    }
}
