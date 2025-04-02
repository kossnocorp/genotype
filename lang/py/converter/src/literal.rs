use genotype_lang_py_tree::{PYContextResolve, PYLiteral};
use genotype_parser::tree::GTLiteral;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYLiteral> for GTLiteral {
    fn convert(&self, context: &mut PYConvertContext) -> PYLiteral {
        match self {
            GTLiteral::Null(_) => PYLiteral::None,
            GTLiteral::Boolean(_, value) => PYLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => PYLiteral::Integer(*value),
            GTLiteral::Float(_, value) => PYLiteral::Float(*value),
            GTLiteral::String(_, value) => PYLiteral::String(value.clone()),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYLiteral::None,
            GTLiteral::Null((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYLiteral::Boolean(true),
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYLiteral::Integer(-123),
            GTLiteral::Integer((0, 0).into(), -123).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYLiteral::Float(1.23),
            GTLiteral::Float((0, 0).into(), 1.23).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYLiteral::String("Hello, world!".into()),
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Default::default();
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
            PYLiteral::Boolean(false)
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Typing, "Literal".into())]
        );
    }
}
