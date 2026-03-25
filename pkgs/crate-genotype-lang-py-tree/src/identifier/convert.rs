use crate::prelude::internal::*;

impl PyConvert<PyIdentifier> for GtIdentifier {
    fn convert(&self, context: &mut PyConvertContext) -> PyIdentifier {
        PyIdentifier(context.resolve_identifier(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut PyConvertContext::default()),
            @r#"PyIdentifier("Foo")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PyConvertResolve::default();
        resolve.identifiers.insert(
            GtIdentifier::new((0, 0).into(), "Foo".into()),
            GtIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        let mut context = PyConvertContext::new(resolve.clone(), Default::default());
        assert_ron_snapshot!(
            GtIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut context),
            @r#"PyIdentifier("foo.Bar")"#
        );
    }
}
