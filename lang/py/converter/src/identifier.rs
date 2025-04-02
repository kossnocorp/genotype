use genotype_lang_py_tree::identifier::PYIdentifier;
use genotype_parser::tree::identifier::GTIdentifier;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut PYConvertContext) -> PYIdentifier {
        PYIdentifier(context.resolve_identifier(self))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            PYIdentifier("Foo".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PYConvertResolve::default();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        let mut context = PYConvertContext::new(resolve.clone(), Default::default(), None);
        assert_eq!(
            PYIdentifier("foo.Bar".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut context),
        );
    }
}
