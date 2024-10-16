use genotype_lang_py_tree::{definition::PYDefinition, PYLiteral};
use genotype_parser::tree::GTLiteral;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYLiteral> for GTLiteral {
    fn convert<HoistFn>(&self, _resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYLiteral
    where
        HoistFn: Fn(PYDefinition),
    {
        match self {
            GTLiteral::Boolean(_, value) => PYLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => PYLiteral::Integer(*value),
            GTLiteral::Float(_, value) => PYLiteral::Float(*value),
            GTLiteral::String(_, value) => PYLiteral::String(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYLiteral::Boolean(true),
            GTLiteral::Boolean((0, 0).into(), true).convert(&PYConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            PYLiteral::Integer(-123),
            GTLiteral::Integer((0, 0).into(), -123).convert(&PYConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            PYLiteral::Float(1.23),
            GTLiteral::Float((0, 0).into(), 1.23).convert(&PYConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            PYLiteral::String("Hello, world!".into()),
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&PYConvertResolve::new(), &|_| {}),
        );
    }
}
