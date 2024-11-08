use genotype_lang_rs_config::RSVersion;

use crate::*;

use super::RSTuple;

impl RSContextResolve for RSTuple {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        if context.is_version(RSVersion::Legacy) {
            context.import(RSDependency::Typing, "Tuple".into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use genotype_lang_rs_config::RSVersion;
    use mock::RSContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = RSContextMock::default();
        let tuple = RSTuple {
            descriptors: vec![RSPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = RSContextMock::new(RSVersion::Legacy);
        let tuple = RSTuple {
            descriptors: vec![RSPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "Tuple".into())],
        );
    }
}
