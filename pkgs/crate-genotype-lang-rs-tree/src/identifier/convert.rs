use crate::prelude::internal::*;

impl RsConvert<RsIdentifier> for GtIdentifier {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsIdentifier> {
        Ok(RsIdentifier(context.resolve_identifier(self).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"RsIdentifier("Foo")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = RsConvertResolve::default();
        resolve.identifiers.insert(
            GtIdentifier::new((0, 0).into(), "Foo".into()),
            GtIdentifier::new((0, 0).into(), "foo::Bar".into()),
        );
        let mut context = RsConvertContext::new(
            "module".into(),
            resolve.clone(),
            Default::default(),
            Default::default(),
        );
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut context)
                .unwrap(),
            @r#"RsIdentifier("foo::Bar")"#
        );
    }
}
