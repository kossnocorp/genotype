use genotype_lang_py_tree::{PYAny, PYContextResolve};
use genotype_parser::{tree::primitive::GTPrimitive, GTAny};

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYAny> for GTAny {
    fn convert(&self, resolve: &mut PYConvertContext) -> PYAny {
        PYAny.resolve(resolve)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::PYDependency;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::{tree::primitive::GTPrimitive, GTAny};

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
            vec![(PYDependency::Typing, "Any".into())]
        );
    }
}
