use genotype_lang_rs_tree::{RSAny, RSContextResolve};
use genotype_parser::GTAny;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSAny> for GTAny {
    fn convert(&self, resolve: &mut RSConvertContext) -> RSAny {
        RSAny.resolve(resolve)
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
            GTAny((0, 0).into()).convert(&mut RSConvertContext::default()),
            RSAny
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::default();
        assert_eq!(GTAny((0, 0).into(),).convert(&mut context), RSAny);
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "Any".into())]
        );
    }
}
