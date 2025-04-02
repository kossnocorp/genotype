use genotype_lang_rs_tree::identifier::RSIdentifier;
use genotype_parser::tree::identifier::GTIdentifier;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSIdentifier> {
        Ok(RSIdentifier(context.resolve_identifier(self)))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::resolve::RSConvertResolve;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            RSIdentifier("Foo".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = RSConvertResolve::default();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo::Bar".into()),
        );
        let mut context =
            RSConvertContext::new("module".into(), resolve.clone(), Default::default(), None);
        assert_eq!(
            RSIdentifier("foo::Bar".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut context)
                .unwrap(),
        );
    }
}
