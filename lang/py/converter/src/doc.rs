use genotype_lang_py_tree::doc::PYDoc;
use genotype_parser::tree::doc::GTDoc;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDoc> for GTDoc {
    fn convert(&self, _context: &mut PYConvertContext) -> PYDoc {
        PYDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYDoc("Hello, world!".into()),
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&mut PYConvertContext::default()),
        );
    }
}
