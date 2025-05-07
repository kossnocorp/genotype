use crate::prelude::internal::*;

impl RSConvert<RSIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSIdentifier> {
        Ok(RSIdentifier(context.resolve_identifier(self)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        let mut context = RSConvertContext::new(
            "module".into(),
            resolve.clone(),
            Default::default(),
            Default::default(),
        );
        assert_eq!(
            RSIdentifier("foo::Bar".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut context)
                .unwrap(),
        );
    }
}
