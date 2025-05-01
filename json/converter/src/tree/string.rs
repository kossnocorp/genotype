use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjString {
    fn to_tree_with_context(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive::String(Default::default())
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let string = GtjString {
            r#type: GtjStringTypeString,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::String(Default::default()),
            string.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let string = GtjString {
            r#type: GtjStringTypeString,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::String(Default::default())),
            string.to_tree_with_context(&mut Default::default()),
        );
    }
}
