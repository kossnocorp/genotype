use crate::prelude::internal::*;

impl TSConvert<TSIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut TSConvertContext) -> TSIdentifier {
        TSIdentifier(context.resolve_identifier(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GTIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut Default::default()),
            @r#"TSIdentifier("Foo")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        assert_ron_snapshot!(
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut TSConvertContext::new(resolve, Default::default())),
            @r#"TSIdentifier("foo.Bar")"#
        );
    }
}
