use genotype_lang_ts_tree::{definition::TSDefinition, TSLiteral};
use genotype_parser::tree::GTLiteral;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSLiteral> for GTLiteral {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSLiteral
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTLiteral::Boolean(_, value) => TSLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => TSLiteral::Integer(*value),
            GTLiteral::Float(_, value) => TSLiteral::Float(*value),
            GTLiteral::String(_, value) => TSLiteral::String(value.clone()),
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
            TSLiteral::Boolean(true),
            GTLiteral::Boolean((0, 0).into(), true).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSLiteral::Integer(-123),
            GTLiteral::Integer((0, 0).into(), -123).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSLiteral::Float(1.23),
            GTLiteral::Float((0, 0).into(), 1.23).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSLiteral::String("Hello, world!".into()),
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
