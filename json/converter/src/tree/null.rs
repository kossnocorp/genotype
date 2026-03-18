use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjNull {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive::Null(Default::default())
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjNull {
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
        let null = GtjNull {
            r#type: GtjNullTypeNull,
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = null.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"Primitive(Null(GTSpan(0, 0)))");

        let null_tree: GTPrimitive = null.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(null_tree, @"Null(GTSpan(0, 0))");
    }
}
