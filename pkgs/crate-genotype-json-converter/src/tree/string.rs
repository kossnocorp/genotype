use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjString {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive {
            span: Default::default(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::String,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjString {
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
        let string = GtjString {
            r#type: GtjStringTypeString,
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = string.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Primitive(GTPrimitive(
          span: GTSpan(0, 0),
          kind: String,
          doc: None,
          attributes: [],
        ))
        ");

        let string_tree: GTPrimitive = string.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(string_tree, @"
        GTPrimitive(
          span: GTSpan(0, 0),
          kind: String,
          doc: None,
          attributes: [],
        )
        ");
    }
}
