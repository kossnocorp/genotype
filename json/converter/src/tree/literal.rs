use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTLiteral> for GtjLiteral {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTLiteral {
        match &self.value {
            GtjLiteralValue::Null => GTLiteral::Null(Default::default()),
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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::Null,
        };

        let descriptor_tree: GTDescriptor = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"Literal(Null(GTSpan(0, 0)))");

        let literal_tree: GTLiteral = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(literal_tree, @"Null(GTSpan(0, 0))");
    }
}
