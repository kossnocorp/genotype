use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtPrimitive> for GtjBoolean {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GtPrimitive {
        GtPrimitive {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Boolean,
        }
    }
}

impl GtjTreeConvert<GtDescriptor> for GtjBoolean {
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
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };

        let descriptor_tree: GtDescriptor = boolean.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Primitive(GtPrimitive(
          span: GtSpan(0, 0),
          kind: Boolean,
          doc: None,
          attributes: [],
        ))
        ");

        let boolean_tree: GtPrimitive = boolean.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(boolean_tree, @"
        GtPrimitive(
          span: GtSpan(0, 0),
          kind: Boolean,
          doc: None,
          attributes: [],
        )
        ");
    }
}
