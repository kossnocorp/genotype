use crate::*;

use super::RSEnum;

impl RSContextResolve for RSEnum {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        context.import(RSDependency::Serde, "Deserialize".into());
        context.import(RSDependency::Serde, "Serialize".into());
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
        let mut context = RSContextMock::new(RSVersion::Legacy);
        let union = RSEnum {
            doc: None,
            attributes: vec![],
            name: "Union".into(),
            variants: vec![],
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (RSDependency::Serde, "Deserialize".into()),
                (RSDependency::Serde, "Serialize".into())
            ]
        );
    }
}
