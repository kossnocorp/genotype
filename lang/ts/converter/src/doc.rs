use genotype_lang_ts_tree::doc::TSDoc;
use genotype_parser::tree::doc::GTDoc;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSDoc> for GTDoc {
    fn convert(&self, _context: &mut TSConvertContext) -> TSDoc {
        TSDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSDoc("Hello, world!".into()),
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&mut Default::default()),
        );
    }
}
