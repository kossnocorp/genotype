use genotype_lang_py_tree::{definition::PYDefinition, doc::PYDoc};
use genotype_parser::tree::doc::GTDoc;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYDoc> for GTDoc {
    fn convert<HoistFn>(&self, _resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYDoc
    where
        HoistFn: Fn(PYDefinition),
    {
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
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&PYConvertResolve::new(), &|_| {}),
        );
    }
}
