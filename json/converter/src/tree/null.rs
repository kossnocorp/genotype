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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let null = GtjNull {
            r#type: GtjNullTypeNull,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Null(Default::default()),
            null.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let null = GtjNull {
            r#type: GtjNullTypeNull,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Null(Default::default())),
            null.to_tree_with_context(&mut Default::default()),
        );
    }
}
