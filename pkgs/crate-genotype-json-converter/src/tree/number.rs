use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtPrimitive> for GtjNumber {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GtPrimitive {
        GtPrimitive {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Number,
        }
    }
}

impl GtjTreeConvert<GtDescriptor> for GtjNumber {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtDescriptor {
        GtDescriptor::Primitive(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let number = GtjNumber {
            r#type: GtjNumberTypeNumber,
            name: None,
            doc: None,
        };

        let descriptor_tree: GtDescriptor = number.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Primitive(GtPrimitive(
          span: GtSpan(0, 0),
          kind: Number,
          doc: None,
          attributes: [],
        ))
        ");

        let number_tree: GtPrimitive = number.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(number_tree, @"
        GtPrimitive(
          span: GtSpan(0, 0),
          kind: Number,
          doc: None,
          attributes: [],
        )
        ");
    }
}
