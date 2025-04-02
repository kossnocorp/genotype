use genotype_lang_ts_tree::TSLiteral;
use genotype_parser::tree::GTLiteral;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSLiteral> for GTLiteral {
    fn convert(&self, _context: &mut TSConvertContext) -> TSLiteral {
        match self {
            GTLiteral::Null(_) => TSLiteral::Null,
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
            TSLiteral::Null,
            GTLiteral::Null(Default::default()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSLiteral::Boolean(true),
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut Default::default()),
        );
        assert_eq!(
            TSLiteral::Integer(-123),
            GTLiteral::Integer((0, 0).into(), -123).convert(&mut Default::default()),
        );
        assert_eq!(
            TSLiteral::Float(1.23),
            GTLiteral::Float((0, 0).into(), 1.23).convert(&mut Default::default()),
        );
        assert_eq!(
            TSLiteral::String("Hello, world!".into()),
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&mut Default::default()),
        );
    }
}
