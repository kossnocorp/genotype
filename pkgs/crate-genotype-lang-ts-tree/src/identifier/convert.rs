use crate::prelude::internal::*;

impl TsConvert<TsIdentifier> for GtIdentifier {
    fn convert(&self, context: &mut TsConvertContext) -> TsIdentifier {
        TsIdentifier(context.resolve_identifier(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut Default::default()),
            @r#"TsIdentifier("Foo")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TsConvertResolve::new();
        resolve.identifiers.insert(
            GtIdentifier::new((0, 0).into(), "Foo".into()),
            GtIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut TsConvertContext::new(resolve, Default::default())),
            @r#"TsIdentifier("foo.Bar")"#
        );
    }
}
