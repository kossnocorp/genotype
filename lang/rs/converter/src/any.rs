use genotype_lang_rs_tree::{RSAny, RSContext, RSDependency};
use genotype_parser::GTAny;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSAny> for GTAny {
    fn convert(&self, resolve: &mut RSConvertContext) -> Result<RSAny> {
        resolve.import(RSDependency::Runtime, "Any".into());
        Ok(RSAny)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::RSDependency;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::GTAny;

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
            vec![(RSDependency::Runtime, "Any".into())]
        );
    }
}
