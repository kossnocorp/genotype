use genotype_lang_rs_tree::{RSContext, RSDependency, RSReference};
use genotype_parser::GTAny;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSReference> for GTAny {
    fn convert(&self, resolve: &mut RSConvertContext) -> Result<RSReference> {
        resolve.import(RSDependency::SerdeJson, "Value".into());
        Ok(RSReference::new("Value".into()))
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
            RSReference::new("Value".into())
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTAny((0, 0).into(),).convert(&mut context).unwrap(),
            RSReference::new("Value".into())
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::SerdeJson, "Value".into())]
        );
    }
}
