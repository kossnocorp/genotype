use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjNumber {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Number,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjNumber {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Primitive(self.to_tree_with_context(context))
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

        let descriptor_tree: GTDescriptor = number.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Primitive(GTPrimitive(
          span: GTSpan(0, 0),
          kind: Number,
          doc: None,
          attributes: [],
        ))
        ");

        let number_tree: GTPrimitive = number.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(number_tree, @"
        GTPrimitive(
          span: GTSpan(0, 0),
          kind: Number,
          doc: None,
          attributes: [],
        )
        ");
    }
}
