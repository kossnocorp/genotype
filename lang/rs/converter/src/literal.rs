use genotype_lang_rs_tree::{RSContextResolve, RSLiteral};
use genotype_parser::tree::GTLiteral;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSLiteral> for GTLiteral {
    fn convert(&self, context: &mut RSConvertContext) -> RSLiteral {
        match self {
            GTLiteral::Boolean(_, value) => RSLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => RSLiteral::Integer(*value),
            GTLiteral::Float(_, value) => RSLiteral::Float(*value),
            GTLiteral::String(_, value) => RSLiteral::String(value.clone()),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSLiteral::Boolean(true),
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSLiteral::Integer(-123),
            GTLiteral::Integer((0, 0).into(), -123).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSLiteral::Float(1.23),
            GTLiteral::Float((0, 0).into(), 1.23).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSLiteral::String("Hello, world!".into()),
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Default::default();
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
            RSLiteral::Boolean(false)
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "Literal".into())]
        );
    }
}
