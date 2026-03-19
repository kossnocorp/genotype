use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTLiteral> for GtjLiteral {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTLiteral {
        let value = match &self.value {
            GtjLiteralValue::Null => GTLiteralValue::Null,
            GtjLiteralValue::Boolean(boolean) => GTLiteralValue::Boolean(boolean.clone()),
            GtjLiteralValue::Number(number) => GTLiteralValue::Float(number.clone()),
            GtjLiteralValue::String(string) => GTLiteralValue::String(string.clone()),
        };
        GTLiteral {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            value,
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
        assert_ron_snapshot!(descriptor_tree, @"
        Literal(GTLiteral(
          span: GTSpan(0, 0),
          doc: None,
          attributes: [],
          value: Null,
        ))
        ");

        let literal_tree: GTLiteral = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(literal_tree, @"
        GTLiteral(
          span: GTSpan(0, 0),
          doc: None,
          attributes: [],
          value: Null,
        )
        ");
    }
}
