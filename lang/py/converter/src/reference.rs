use genotype_lang_py_tree::{definition::PYDefinition, reference::PYReference};
use genotype_parser::tree::reference::GTReference;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYReference> for GTReference {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYReference
    where
        HoistFn: Fn(PYDefinition),
    {
        // [TODO] Resolve the reference properly
        PYReference::new(self.1.convert(resolve, hoist), true)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::GTIdentifier;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYReference::new("Name".into(), true),
            GTReference(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&PYConvertResolve::new(), &|_| {}),
        );
    }
}
