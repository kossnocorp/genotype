use crate::prelude::internal::*;

impl TSConvert<TSIdentifier> for GTIdentifier {
    fn convert(&self, context: &mut TSConvertContext) -> TSIdentifier {
        TSIdentifier(context.resolve_identifier(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            TSIdentifier("Foo".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into()).convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve.identifiers.insert(
            GTIdentifier::new((0, 0).into(), "Foo".into()),
            GTIdentifier::new((0, 0).into(), "foo.Bar".into()),
        );
        assert_eq!(
            TSIdentifier("foo.Bar".into()),
            GTIdentifier::new((0, 0).into(), "Foo".into())
                .convert(&mut TSConvertContext::new(resolve, None)),
        );
    }
}
