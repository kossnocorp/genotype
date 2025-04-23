use crate::prelude::internal::*;

impl RSConvert<RSAny> for GTAny {
    fn convert(&self, resolve: &mut RSConvertContext) -> Result<RSAny> {
        resolve.add_import(RSDependencyIdent::Runtime, "Any".into());
        Ok(RSAny)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTAny((0, 0).into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSAny
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(GTAny((0, 0).into(),).convert(&mut context).unwrap(), RSAny);
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependencyIdent::Runtime, "Any".into())]
        );
    }
}
