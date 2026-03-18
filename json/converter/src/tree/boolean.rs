use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjBoolean {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive::Boolean(Default::default())
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjBoolean {
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
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = boolean.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"Primitive(Boolean(GTSpan(0, 0)))");

        let boolean_tree: GTPrimitive = boolean.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(boolean_tree, @"Boolean(GTSpan(0, 0))");
    }
}
