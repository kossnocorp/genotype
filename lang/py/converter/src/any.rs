use crate::prelude::internal::*;

impl PYConvert<PYAny> for GTAny {
    fn convert(&self, resolve: &mut PYConvertContext) -> PYAny {
        PYAny.resolve(resolve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTAny((0, 0).into()).convert(&mut PYConvertContext::default()),
            PYAny
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_eq!(GTAny((0, 0).into(),).convert(&mut context), PYAny);
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependencyIdent::Typing, "Any".into())]
        );
    }
}
