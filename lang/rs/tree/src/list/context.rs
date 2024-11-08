use genotype_lang_rs_config::RSVersion;

use crate::*;

use super::RSList;

impl RSContextResolve for RSList {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        if context.is_version(RSVersion::Legacy) {
            context.import(RSDependency::Typing, "List".into());
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
        let list = RSList {
            descriptor: RSPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = RSContextMock::new(RSVersion::Legacy);
        let list = RSList {
            descriptor: RSPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "List".into())]
        );
    }
}
