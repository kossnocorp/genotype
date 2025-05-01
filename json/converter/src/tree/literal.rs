use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTLiteral> for GtjLiteral {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTLiteral {
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

impl GtjTreeConvert<GTDescriptor> for GtjLiteral {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Literal(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::Null(()),
        };
        assert_eq!(
            GTLiteral::Null(Default::default()),
            literal.to_tree_with_context(&mut Default::default()),
        );
    }
}
