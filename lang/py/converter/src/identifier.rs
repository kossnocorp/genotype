use genotype_lang_py_tree::{definition::PYDefinition, identifier::PYIdentifier};
use genotype_parser::tree::identifier::GTIdentifier;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYIdentifier> for GTIdentifier {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYIdentifier
    where
        HoistFn: Fn(PYDefinition),
    {
        PYIdentifier(resolve.identifiers.get(&self).unwrap_or(&self).1.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            PYIdentifier("Foo".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&PYConvertResolve::new(), &|_| {}),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PYConvertResolve::new();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        assert_eq!(
            PYIdentifier("foo.Bar".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into()).convert(&resolve, &|_| {}),
        );
    }
}
