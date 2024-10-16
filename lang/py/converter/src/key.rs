use genotype_lang_py_tree::{definition::PYDefinition, key::PYKey};
use genotype_parser::tree::key::GTKey;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYKey> for GTKey {
    fn convert<HoistFn>(&self, _resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYKey
    where
        HoistFn: Fn(PYDefinition),
    {
        PYKey(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYKey("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&PYConvertResolve::new(), &|_| {}),
        );
    }
}
