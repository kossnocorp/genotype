use genotype_lang_rs_config::RSVersion;

use crate::{RSContext, RSContextResolve, RSDependency};

use super::RSAlias;

impl RSContextResolve for RSAlias {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        if context.is_version(RSVersion::Legacy) {
            context.import(RSDependency::Typing, "TypeAlias".into());
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
        let alias = RSAlias {
            doc: None,
            name: "Foo".into(),
            descriptor: RSPrimitive::String.into(),
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = RSContextMock::new(RSVersion::Legacy);
        let alias = RSAlias {
            doc: None,
            name: "Foo".into(),
            descriptor: RSPrimitive::String.into(),
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "TypeAlias".into())]
        );
    }
}
