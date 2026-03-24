use crate::prelude::internal::*;

impl PYConvert<PYIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut PYConvertContext) -> PYIdentifier {
        PYIdentifier(context.resolve_identifier(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut PYConvertContext::default()),
            @r#"PYIdentifier("Foo")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PYConvertResolve::default();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        let mut context = PYConvertContext::new(resolve.clone(), Default::default());
        assert_ron_snapshot!(
            GTIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut context),
            @r#"PYIdentifier("foo.Bar")"#
        );
    }
}
