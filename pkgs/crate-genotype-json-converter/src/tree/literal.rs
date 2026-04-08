use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtLiteral> for GtjLiteral {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GtLiteral {
        let value = match &self.value {
            GtjLiteralValue::Null => GtLiteralValue::Null,
            GtjLiteralValue::Boolean(boolean) => GtLiteralValue::Boolean(*boolean),
            GtjLiteralValue::Number(number) => GtLiteralValue::Float(*number),
            GtjLiteralValue::String(string) => GtLiteralValue::String(string.clone()),
        };
        GtLiteral {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            value,
        }
    }
}

impl GtjTreeConvert<GtDescriptor> for GtjLiteral {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtDescriptor {
        GtDescriptor::Literal(self.to_tree_with_context(context))
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

        let descriptor_tree: GtDescriptor = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Literal(GtLiteral(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          value: Null,
        ))
        ");

        let literal_tree: GtLiteral = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(literal_tree, @"
        GtLiteral(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          value: Null,
        )
        ");
    }
}
