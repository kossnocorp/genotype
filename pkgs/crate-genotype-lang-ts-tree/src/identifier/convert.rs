use crate::prelude::internal::*;

impl TsConvert<TsIdentifier> for GtIdentifier {
    fn convert(&self, context: &mut TsConvertContext) -> TsIdentifier {
        TsIdentifier(context.resolve_identifier(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            convert_node(GtIdentifier::new((0, 0).into(), "Foo".into())),
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
        let mut context = TsConvertContext::new(resolve, &Default::default());
        assert_ron_snapshot!(
            convert_node_with(
                GtIdentifier::new((0, 0).into(), "Foo".into()),
                &mut context,
            ),
            @r#"TsIdentifier("foo.Bar")"#
        );
    }
}
