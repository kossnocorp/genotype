use genotype_json_tree::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTLiteral> for GtjLiteral {
    fn convert(&self, _context: &mut GtjConvertContext) -> GTLiteral {
        match &self.value {
            GtjLiteralValue::Null(_) => GTLiteral::Null(Default::default()),
            GtjLiteralValue::Boolean(boolean) => {
                GTLiteral::Boolean(Default::default(), boolean.clone())
            }
            GtjLiteralValue::Number(number) => GTLiteral::Float(Default::default(), number.clone()),
            GtjLiteralValue::String(string) => {
                GTLiteral::String(Default::default(), string.clone())
            }
        }
    }
}

impl GtjConvert<GTDescriptor> for GtjLiteral {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Literal(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::Null(()),
        };
        assert_eq!(
            GTLiteral::Null(Default::default()),
            literal.convert(&mut Default::default()),
        );
    }
}
